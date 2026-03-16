use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    model: Llama,
    chat_session: Chat<Llama>
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        // creates the chat session so we can store it into the chatbot struct
        let chat_session = model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        return ChatbotV2 {
            model: model,  // we store the model object just in case
            chat_session: chat_session  // we also store the chat session to remember context
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.

        // this is kinda like a work in progress out put that gets wrapped
        let asynchronous_output = self.chat_session.add_message(message);
        
        // notice lack of (), await is not a function; it is a special keyword!
        let output = asynchronous_output.await;

        return output.unwrap();
    }
}