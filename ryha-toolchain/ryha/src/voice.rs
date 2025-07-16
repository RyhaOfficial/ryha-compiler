// ryha-toolchain/ryha/src/voice.rs

pub struct VoiceCommandParser {}

impl VoiceCommandParser {
    pub fn new() -> Self {
        VoiceCommandParser {}
    }

    pub fn parse(&self, command: &str) -> Option<String> {
        if command == "build" {
            Some("build".to_string())
        } else {
            None
        }
    }
}
