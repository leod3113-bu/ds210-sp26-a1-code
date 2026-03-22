use kalosm::language::*;

// Look at the docs for std::fs
// https://doc.rust-lang.org/std/fs/index.html
// std::fs provides functions that write to a file, read from a file,
// check if a file exists, etc.
use std::fs;

// LlamaChatSession provides helpful functions for loading and storing sessions.
// Look at https://docs.rs/kalosm/latest/kalosm/language/trait.ChatSession.html#saving-and-loading-sessions
// for some examples!

// Implement this
pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) {
    // look at fs::write(...)
    unimplemented!("Saving chat session to file {filename}");
}

// Implement this
pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
    let chat_File= fs::read (filename).unwrap(); // read funtion returns a Result Vec<u8> so we have to unwrap the Result
    // look at fs::read(...)
    // also look at
    let session_load= LlamaChatSession::from_bytes(&chat_File);
    return match session_load {
        Ok(session) => Some(session), // Ok grabs the result and turns it into a session which is then wrapped again as an option

        _ => None // retunring empty package
        
    }

}