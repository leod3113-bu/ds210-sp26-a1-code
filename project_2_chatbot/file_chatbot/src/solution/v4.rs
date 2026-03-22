use kalosm::language::*;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // Creates destination filename
        let filename = &format!("{}.txt", username);

        // Creates new chat session
        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        // Attempts to load session from file
        let session = file_library::load_chat_session_from_file(filename);
        if session.is_some() {
            // If it exists, apply it, otherwise ignore it
            chat_session = chat_session.with_session(session.unwrap());
        }

        // Generates the output and returns it to the user
        let output = chat_session.add_message(message).await;
        if output.is_err() {
            // Panics if it failed to generate
            panic!("Failed to generate output!");
        }
        return output.unwrap();
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            },
            Some(session) => {
                // TODO: what should happen here?
                return Vec::new();
            }
        }
    }
}