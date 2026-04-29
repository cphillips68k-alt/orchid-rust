#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessagePayload {
    Text(&'static str),
    Event(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Message {
    pub sender: &'static str,
    pub recipient: &'static str,
    pub payload: MessagePayload,
}

impl Message {
    pub fn new(sender: &'static str, recipient: &'static str, payload: MessagePayload) -> Self {
        Message {
            sender,
            recipient,
            payload,
        }
    }
}

const QUEUE_CAPACITY: usize = 16;

pub struct IpcHub {
    queue: [Option<Message>; QUEUE_CAPACITY],
    len: usize,
}

impl IpcHub {
    pub fn new() -> Self {
        IpcHub {
            queue: [None; QUEUE_CAPACITY],
            len: 0,
        }
    }

    pub fn send(&mut self, message: Message) -> bool {
        if self.len >= QUEUE_CAPACITY {
            return false;
        }
        self.queue[self.len] = Some(message);
        self.len += 1;
        true
    }

    pub fn receive(&mut self, recipient: &'static str) -> Option<Message> {
        for index in 0..self.len {
            if let Some(message) = self.queue[index] {
                if message.recipient == recipient {
                    let result = Some(message);
                    for shift in index..self.len - 1 {
                        self.queue[shift] = self.queue[shift + 1];
                    }
                    self.len -= 1;
                    self.queue[self.len] = None;
                    return result;
                }
            }
        }
        None
    }

    pub fn has_message(&self, recipient: &'static str) -> bool {
        for index in 0..self.len {
            if let Some(message) = self.queue[index] {
                if message.recipient == recipient {
                    return true;
                }
            }
        }
        false
    }
}
