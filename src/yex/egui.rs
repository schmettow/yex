// use std::sync::mpsc;
use crate::session::Session;
use super::{Arc, Mutex};

#[allow(dead_code)]
pub fn update(session: Arc<Mutex<Session>>){
    // Reading state of session
    let session = session.lock().unwrap();
    let _part = session.part.clone();
}