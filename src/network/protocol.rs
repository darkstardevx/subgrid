#[derive(Debug)]
#[allow(dead_code)] // This tells the compiler: "I know these aren't used yet, ignore it."
pub enum CommandType {
    Ping,
    Chat,
    VoiceData,
    Status,
    Shutdown,
}

#[derive(Debug)]
pub struct Message {
    pub command: CommandType,
    pub sender: String,
    pub payload: String,
}

impl Message {
    pub fn new(command: CommandType, sender: &str, payload: &str) -> Self {
        Self {
            command,
            sender: sender.to_string(),
            payload: payload.to_string(),
        }
    }
}
