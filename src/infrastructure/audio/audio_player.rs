use crate::domain::services::audio_service::AudioServiceInterface;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::RwLock;
use std::time::Duration;

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

                        let url = format!("{}/audio/{}", base_url, word);
                        let bytes = match client.get(&url).send().and_then(|r| r.bytes()) {
                            Ok(b) => b,
                            Err(_) => continue,
                        };

                        let sink = match Sink::try_new(&handle) {
                            Ok(s) => s,
                            Err(_) => continue,
                        };
                        let cursor = Cursor::new(bytes.to_vec());
                        if let Ok(source) = Decoder::new(cursor) {
                            sink.append(source);
                            current_sink = Some(sink);
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
