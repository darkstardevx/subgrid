use crate::network::protocol::{Message, CommandType};

pub struct Scheduler;

impl Scheduler {
pub fn process(msg: Message) {
    match msg.command {
        CommandType::Ping => {
            println!("[{}] Scheduler: Pong!", msg.sender);
        }
        CommandType::Chat => {
            println!("[{}] Chat: {}", msg.sender, msg.payload);
        }
        CommandType::Status => {
            println!("[{}] Scheduler: Reporting status...", msg.sender);
        }
        CommandType::Shutdown => {
            println!("[{}] Scheduler: Initiating shutdown by {}", msg.sender, msg.payload);
        }
        CommandType::VoiceData => {
            println!("[{}] Voice stream from: {}", msg.sender, msg.payload);
        }
    }
}
}
