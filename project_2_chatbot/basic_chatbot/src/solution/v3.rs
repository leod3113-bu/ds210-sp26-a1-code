use std::collections::HashMap;
use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV3 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.
    
    model: Llama,                                // the actual llama model
    chat_sessions: HashMap<String, Chat<Llama>>  // we use a hash map here so we can do chat session lookups
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        // creates the empty hash map so we can store every chat session ever created
        let chat_sessions = HashMap::new();

        return ChatbotV3 {
            // Make sure you initialize your struct members here
            model: model,
            chat_sessions: chat_sessions
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.

        // we pull the chat session from our known sessions
        match self.chat_sessions.get_mut(&username) {
            // if the chat session exists
            Some(chat_session) => {
                // we create the output
                let output = chat_session.add_message(message).await;

                // and return the output
                return output.unwrap();
            },

            // otherwise if it doesn't exist
            None => {
                // we create a new chat session
                let mut chat_session = self.model
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate");

                // we create the output
                let output = chat_session.add_message(message).await;                

                // and save the chat session for later
                self.chat_sessions.insert(username, chat_session);

                // before returning the output to the user
                return output.unwrap();
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        // Extract the chat message history for the given username
        // Hint: think of how you can retrieve the Chat object for that user, when you retrieve it
        // you may want to use https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.session
        // to then retrieve the history!

        // we pull the chat session from our known sessions
        return match self.chat_sessions.get(&username) {
            // if the chat_session exists
            Some(chat_session) => {
                // we get its history, iterate over it and pull the contents, and return it as a vector
                // with credits: https://stackoverflow.com/questions/30026893/how-to-use-a-map-over-vectors
                let history = chat_session.session().unwrap().history();
                return history.iter().map(|chat_message| String::from(chat_message.content())).collect();
            },

            // otherwise we return an empty vector
            None => Vec::new()
        }
    }
}