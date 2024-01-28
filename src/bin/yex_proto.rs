use yex::session::{Participant, Experiment,Session};
//use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::Builder;

fn main() {
    let exp = Experiment::default();
    println!("Exp with {} blocks", exp.blocks.len());
    let part = Participant::default();
    println!("Hello {}.", part.id);
    let session = Arc::new(Mutex::new(Session::new(exp, part)));
    let builder = Builder::new();
    let join_handle = 
        builder.spawn(move || {
            yex::demo(session)
        }).unwrap();
    match join_handle.join() {
        Ok(obs) => 
            {println!("{} observations collected", obs.len())},
        Err(_) => println!("Session failed")
        
    }
}
