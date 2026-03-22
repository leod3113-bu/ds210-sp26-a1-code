use kalosm::language::*;
use rocket::tokio::fs;
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
            .with_system_prompt("The assistant will act like luffy from one piece");

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
        else {
            // Else save the session to the file
            let session = &chat_session.session().unwrap();
            file_library::save_chat_session_to_file(filename, session);
        }
        return output.unwrap();
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        match file_library::load_chat_session_from_file(&filename) {
            // hello world
            None => {
                return Vec::new();
            },
            Some(session) => {
                // We want to return a vector of strings.
                // We iterate through the whole vector
                // and make every content within the vector a string
                // and then collect it to make it a vector
                return session
                    .history()
                    [1..]// so we dont take system prompt into consideration
                    .iter()
                    .map(|chat_message| String::from(chat_message.content()))
                    .collect();
            }
        }
    }


}

