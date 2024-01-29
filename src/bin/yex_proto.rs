use yex::session::{Participant, Experiment,Session};
use yex::output::YexEvent;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{self, Builder};

fn main() {
    // Dummy experiment
    let exp = Experiment::default();
    println!("Exp with {} blocks", exp.blocks.len());
    // Dummy participant
    let part = Participant::default();
    println!("Hello {}.", part.id);
    // Creating the session
    let session = Arc::new(Mutex::new(Session::new(exp, part)));
    // Starting the event recorder channel
    let (event_snd, event_rec)  = channel::<YexEvent>();
    // Detached mock task to receive and print the events
    thread::spawn(move || {loop {match event_rec.recv() {Ok(r) => {println!("{:?}",r)}, _ => {}} }});
    // Building the session thread, because we want it to return a value
    let builder = Builder::new();
    let join_handle = 
        builder.spawn(move || {yex::demo(session, event_snd)}).unwrap();
    match join_handle.join() {
        Ok(obs) => 
            {println!("{} observations collected", obs.len())},
        Err(_) => println!("Session failed")
        
    }
}
