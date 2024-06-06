use crate::managers::Manager;

pub struct ChatManager {

}

impl ChatManager {
    pub fn print(&self) {
        println!("Hello World");
    }
}

impl Manager for ChatManager {
    fn init() -> Self where Self: Sized {
        ChatManager {

        }
    }
}