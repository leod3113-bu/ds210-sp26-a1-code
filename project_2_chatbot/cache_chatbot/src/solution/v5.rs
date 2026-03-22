use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => { 
                let mut new_chat = self.model.chat().with_system_prompt("Act like luffy from one piece"); // if there is no session file we create a new bot
                let session_file= file_library::load_chat_session_from_file(filename); // we try download session file 
                 if session_file.is_some() {
                      new_chat = new_chat.with_session(session_file.unwrap());  // the new chat we create we uploade it with the information from the file
                }
            return new_chat.add_message(message).await.unwrap();
            }
            Some(existing_chat) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                let output_cache = existing_chat.add_message(message).await; // We want to continue the chat with the user. 
                return output_cache.unwrap();

            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                // TODO: The cache does not have the chat. What should you do?
                // Your code goes here.
                return Vec::new();
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                // TODO: The cache has this chat. What should you do?
                // Your code goes here.
                return Vec::new();

            }
        }
    }
}