/// YEX is Yourself Experimentation
/// 
/// This library provides data structures to build 
/// sequential experiments with visual stimuli
/// 
/// By limiting the use case to strictly sequential interaction,
/// no or little async/await will be required. More complex experiments 
/// can still be constructed by using Blocks.
/// 
/// The main run time container object is Session, which is constructed as a ArcMut in main(). In the main program, 
/// a session can be created by combining an Experiment and a Participant.
/// 
/// The hierarchical building blocks are Session --> Block --> Trial --> Stimulus
///
/// The Yex runtime is implemented as a state machine to work with 
/// an immediate Gui, like Egui.
/// 

mod egui;
mod yet;
mod exp;

pub use std::time::{Instant, Duration};
pub use std::thread::sleep;
pub use std::sync::{Arc,Mutex};
pub use std::sync::mpsc::{channel, Sender, Receiver};
pub use output::YexEvent;
//use futures::channel::mpsc;
pub use isolang::Language;

/// Input events
pub type Text = String;
pub type Key = char;

pub enum NaviEvent{Back, Forward, Quit}

pub enum Event {
    Response(),
    InputEvent,
    AdvanceAfter(Duration)
}

/// Demo runtime
/// 
/// cycles through a brief demo experiment
/// Collects virtual responses and completes with a Vector of Observations

use exp::*;
use trial::Observation;
use session::Session;
use output::YexRecord;

pub fn demo(session: Arc<Mutex<Session>>, events_out: Sender<YexRecord>)
        -> Vec<Observation>{

    let mut obs_out: Vec<Observation> = Vec::new();
    let mut session = session.lock().unwrap();
    let mut state = &session.state;
    events_out.send(YexEvent::Session(session.state.clone()).into()).unwrap();
    state = &session::State::Welcome;
    sleep(Duration::from_millis(500));
    for block in session.exp.blocks.iter(){
        let obs 
            = block.clone().run(events_out.clone());
        match obs {
            Some(mut obs) => {obs_out.append(&mut obs);},
            None => {println!("No observations collected")},
        }
    }
    state = &session::State::Goodbye;
    obs_out
}



/// Output
/// 
/// in terms of
/// + event stream
/// + observations

pub mod output {
    use super::exp::{session, block, trial};
    use super::{Key, Instant};

    #[derive(Debug)]
    pub enum YexError {
        FileNotFound(),
        PartInterrupt(),

    }

    #[derive(Debug)]
    pub enum YexEvent {
        Error(YexError),
        Session(session::State),
        Block(block::State),
        Trial(trial::State),
        Stimulus(trial::Stimulus),
        KeyPress(Key),
        Response(trial::Response),
    }

    /// Into from Event to Record
    /// 
    /// simply adds Instant::now() as time stamp
    /// should therefore be used close in time
    /// to when the event arrived.
    impl Into<YexRecord> for YexEvent {
        fn into(self) -> YexRecord {
            YexRecord(Instant::now(), self)
        }
    }

    #[derive(Debug)]
    pub struct YexRecord (pub Instant, pub YexEvent);

    /* use std::fmt::{Display, Formatter, Result};
    impl std::fmt::Display for YexRecord {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut Formatter) -> Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "{}", self.0)
        }
    }*/

}
