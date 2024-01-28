use yex::session::{Participant, Experiment,Session};
pub use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let exp = Experiment::default();
    println!("Exp with {} blocks", exp.blocks.len());
    let part = Participant::default();
    println!("Hello {}.", part.id);
    let session = Arc::new(Mutex::new(Session::new(exp, part)));
    thread::spawn(move || {
        yex::demo(session);
    });
}
