use crate::domain::services::audio_service::AudioServiceInterface;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::RwLock;
use std::time::Duration;

const WORD_PAUSE: Duration = Duration::from_millis(350);

pub enum AudioCommand {
    Play { word: String, base_url: String },
}

#[derive(shaku::Component)]
#[shaku(interface = AudioServiceInterface)]
pub struct AudioPlayer {
    #[shaku(default)]
    base_url: RwLock<Option<String>>,
    #[shaku(default)]
    sender: RwLock<Option<mpsc::Sender<AudioCommand>>>,
}

impl AudioServiceInterface for AudioPlayer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn set_base_url(&self, url: String) {
        let (tx, rx) = mpsc::channel::<AudioCommand>();
        *self.sender.write().unwrap() = Some(tx);
        *self.base_url.write().unwrap() = Some(url.clone());

        let client = reqwest::blocking::Client::new();

        std::thread::spawn(move || {
            let (_stream, handle) = match OutputStream::try_default() {
                Ok(s) => s,
                Err(_) => return,
            };
            let mut current_sink: Option<Sink> = None;

            loop {
                match rx.recv_timeout(Duration::from_millis(50)) {
                    Ok(AudioCommand::Play { word, base_url }) => {
                        if let Some(sink) = current_sink.take() {
                            sink.stop();
                        }

                        match play_audio_text(&client, &handle, &base_url, &word) {
                            Some(sink) => current_sink = Some(sink),
                            None => play_pronunciation_tokens(&client, &handle, &base_url, &word),
                        }
                    }
                    Err(RecvTimeoutError::Timeout) => {
                        if let Some(ref sink) = current_sink {
                            if sink.empty() {
                                current_sink = None;
                            }
                        }
                    }
                    Err(RecvTimeoutError::Disconnected) => break,
                }
            }
        });
    }

    fn has_base_url(&self) -> bool {
        self.base_url.read().unwrap().is_some()
    }

    fn play_word(&self, word: &str) {
        let base_url = match self.base_url.read().unwrap().clone() {
            Some(url) => url,
            None => return,
        };
        if let Some(ref sender) = *self.sender.read().unwrap() {
            let _ = sender.send(AudioCommand::Play {
                word: word.to_string(),
                base_url,
            });
        }
    }
}

pub fn pronunciation_tokens(text: &str) -> Vec<String> {
    text.split(is_pronunciation_separator)
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::to_string)
        .collect()
}

fn is_pronunciation_separator(ch: char) -> bool {
    ch.is_whitespace()
        || matches!(
            ch,
            '.' | ','
                | ';'
                | ':'
                | '!'
                | '?'
                | '/'
                | '\\'
                | '|'
                | '-'
                | '_'
                | '('
                | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | '<'
                | '>'
                | '"'
                | '\''
                | '`'
                | '~'
                | '+'
                | '='
                | '*'
                | '&'
                | '#'
                | '@'
                | '^'
                | '%'
                | '$'
        )
}

fn play_pronunciation_tokens(
    client: &reqwest::blocking::Client,
    handle: &rodio::OutputStreamHandle,
    base_url: &str,
    text: &str,
) {
    pronunciation_tokens(text)
        .into_iter()
        .filter_map(|token| play_audio_text(client, handle, base_url, &token))
        .for_each(|sink| {
            sink.sleep_until_end();
            std::thread::sleep(WORD_PAUSE);
        });
}

fn play_audio_text(
    client: &reqwest::blocking::Client,
    handle: &rodio::OutputStreamHandle,
    base_url: &str,
    text: &str,
) -> Option<Sink> {
    let encoded_text = urlencoding::encode(text);
    let url = format!("{}/audio/{}", base_url, encoded_text);
    let bytes = client
        .get(&url)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .and_then(|response| response.bytes())
        .ok()?;
    let sink = Sink::try_new(handle).ok()?;
    let cursor = Cursor::new(bytes.to_vec());
    let source = Decoder::new(cursor).ok()?;
    sink.append(source);
    Some(sink)
}
