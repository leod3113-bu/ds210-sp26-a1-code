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
        // Initializes chat fetching
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        // Tries to fetch the chat from cache
        match cached_chat {
            // On cache miss
            None => {
                // Uses the file library to load the history from string
                let session_attempt = file_library::load_chat_session_from_file(filename);
                
                // If the session does not exist previously, we return an empty history
                if session_attempt.is_none() {
                    return Vec::new();
                }

                // If it does, we grab the session
                let session = session_attempt.unwrap();

                // Pulls the history from the session, slicing 1 to ignore system prompt
                let history = &session.history()[1..];

                // Maps the history to get the content strings
                let contents = history.iter().map(|msg| String::from(msg.content())).collect();
                
                // Returns contents
                return contents;
            }

            // On cache hit
            Some(chat_session) => {
                // Pulls the session from the chat_session
                println!("get_history: {username} is in the cache! Nice!");
                let session = chat_session.session().unwrap();
                
                // Pulls the history from the session, slicing 1 to ignore system prompt
                let history = &session.history()[1..];

                // Maps the history to get the content strings
                let contents = history.iter().map(|msg| String::from(msg.content())).collect();
                
                // Returns contents
                return contents;
            }
        }
    }
}