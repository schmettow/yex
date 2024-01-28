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
    // let mut obs: Option<Vec<Observation>>;
    
    let builder = Builder::new();
    let join_handle = 
        builder.spawn(move || {
            yex::demo(session)
        }).unwrap().join();
    match join_handle {
        Ok(obs) => 
            {println!("{} observations collected", obs.len())},
        Err(_) => println!("Session failed")
        
    }
}
