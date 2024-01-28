// use std::sync::mpsc;
use crate::session::Session;
use super::{Arc, Mutex};

pub fn update(session: Arc<Mutex<Session>>){
    // Reading state of session
    let session = session.lock().unwrap();
    let part = session.part.clone();
}