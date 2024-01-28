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
mod egui;

pub use std::time::{Instant, Duration};
pub use std::thread::sleep;
pub use std::sync::{Arc,Mutex};
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

use session::*;
use trial::Observation;
pub fn demo(session: Arc<Mutex<Session>>) -> Vec<trial::Observation>{
    let mut obs_out: Vec<Observation> = Vec::new();
    let mut session = session.lock().unwrap();
    session.state = State::Welcome;
    println!("Welcome");
    sleep(Duration::from_millis(500));
    for block in &mut session.exp.blocks{
        println!("Block {:?}", block.id);
        let obs = block.run();
        match obs {
            Some(mut obs) => {obs_out.append(&mut obs);},
            None => {println!()},
        }
    }
    session.state = State::Goodbye;
    return obs_out
}


/// Building sessions
/// 
/// A session is the whole encounter of a participant with an experiment.
/// 
/// + composed of a Participant and Experiment object.
/// + runs linearly through the steps of the experiment
/// + sending high-level events

 
pub mod session {
    use super::{Instant, Language, Text};
    use super::block::Block;

    pub struct Session {
        pub id: Instant,
        pub part: Participant,
        pub exp: Experiment,
        pub state: State,
    }

    pub enum State {
        Init,
        Welcome,
        Consent,
        Demographics,
        Blocks(Block),
        Goodbye
    }

    impl Session {
        pub fn new(exp: Experiment, part: Participant) -> Self{
            Session{id: Instant::now(),
                    part: part,
                    exp: exp,
                    state: State::Init}
        }
    }


    #[derive(Clone)]
    pub struct Participant {
        pub id: usize,
        pub age: i8,
        pub gender: Gender,
        pub language: Language,
    }

    impl Default for Participant {
        fn default() -> Self {
            Self { id: 0, age: 42, gender: Gender::Straight(Sex::Male), language: Language::default() }
        }
    }

    #[derive(Clone)]
    pub enum Sex {
        Male,
        Female,
    }

    #[derive(Clone)]
    pub enum Gender {
        Straight(Sex),
        Gay(Sex),
        Bi(Sex),
        Asexual(Sex)
    }


    /// Experiments are composed of blocks
    /// 
    /// An Experiment is a container for trials arranged in blocks.
    /// 
    /// data-only class as Session is doing the run()


    #[derive(Clone)]
    pub struct Experiment {
        pub id: String,
        pub blocks: Vec<Block>,
        pub instructions: Text,
        pub random: bool,
    }

    impl Default for Experiment {
        fn default() -> Self {
            Self {  id: "Stroop".into(), 
                    blocks: vec![Block::default();2],
                    instructions: "Say the color of the word!".into(),
                    random: false,}
        }
}



}


/// Block level

pub mod block { 
    use super::trial::{Trial, Observation};
    use super::{Duration, Instant, sleep, Key, Text};

    /// A Block is a sequences of Trials
    /// 
    /// with a prelude and relax frame.
    /// 
    /// + running through trials
    /// + sending block-level events
    /// 
    #[derive(Clone)]
    pub struct Block{
        pub id: Instant,
        pub trials: Vec<Trial>,
        pub random: bool,
        pub prelude: Prelude,
        pub relax: Relax,
        pub state: State,
    }

    
    impl Default for Block {
        fn default() -> Self {
            let trials = vec![Trial::default(); 3];
            Block{  id: Instant::now(),
                    trials: trials, 
                    random: false, 
                    prelude: Prelude::Blank(Duration::from_millis(1000)),
                    relax: Relax::Wait(Duration::from_millis(2000)),
                    state: State::Init,
                }
        }
    }

    #[derive(Clone, PartialEq)]    
    /// Block states
    /// 
    pub enum State {
        Init,
        Prelude,
        Present(usize), // trial number
        Relax
    }

    /// Preludes types for Blocks
    /// 
    #[derive(Clone, PartialEq, Debug)]
    pub enum Prelude {
        Now,
        Blank(Duration),
        Instruct(Duration, Text),
        InstructKeys(Vec<Key>, Text)
    }

    /// Relax types for Blocks
    ///
    #[derive(Clone)]
    pub enum Relax {
        Now,
        Wait(Duration),
        Keys(Vec<Key>),
        KeysMaxWait(Vec<Key>, Duration)
    }

    
    impl Block {
    /// Run a block
    /// 
    /// runs through one block and its trials
    /// returns a vector of Observations (Trial + Response)
    /// 1. initialize the output vector
    /// 2. do the prelude
    /// 3. cycle through trials and 
    /// 4. Run the relax period
    /// 
        pub fn run(&mut self) -> Option<Vec<Observation>> {
            let mut out: Vec<Observation> = Vec::new();
            self.state = State::Prelude;
            println!("Block Prelude");          
            match self.prelude.clone() {
                Prelude::Now
                    => {},
                Prelude::Blank(dur)
                    => {sleep(dur)},
                Prelude::Instruct(dur, _) 
                    => {sleep(dur);},
                _   => todo!(),
            }

            for trial in self.trials.clone(){
                println!("Trial");
                let obs = trial.clone().run();
                print!("Response...");
                match obs {
                    None => {},
                    Some(obs) => {out.push(obs); println!("Collected");}
                }
            }

            self.state = State::Relax;
            match self.relax {
                Relax::Now => {},
                Relax::Wait(dur) 
                    => {sleep(dur);},
                _   => {todo!();}
            }
            Some(out)
        }
    }


}


/// Trial-level
/// 

pub mod trial { 
    use super::{Duration, sleep, Key};

    /// A trial is a Stimulus with a Prelude and Advance frame
    /// 

    #[derive(Clone, PartialEq)]
    pub struct Trial {
        pub prelude: Prelude,
        pub stimulus: Stimulus,
        pub advance: Advance,
        pub state: State
    }
    
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum State {
        Init,
        Prelude,
        Present,
        Feedback
    }
    
    impl Default for Trial {
        fn default() -> Self {
            Self {  state: State::Init,
                    prelude: Prelude::Blank(Duration::from_micros(500)) ,
                    stimulus: Stimulus::Blank(Duration::from_micros(500)),
                    advance: Advance::Wait(Duration::from_millis(500))}
        }
    }
    
    impl Trial {
        pub fn prepare(&mut self) -> Self{
            self.stimulus.load();
            self.clone()
        }
        pub fn run(&mut self) -> Option<Observation> {
            self.prepare();
            self.state = State::Prelude;
            match self.prelude {
                Prelude::Now => {},
                Prelude::Blank(dur) | Prelude::Fix(dur) 
                    => {sleep(dur);},
                Prelude::Prime(_,_) => todo!(),
            }
            self.state = State::Present;
            // Emulating the incoming response from the participant.
            // 
            // Here we will have time-outs and user events intermixed.
            // Would be nice to have some async here, maybe 
            // block_on(select())
            sleep(Duration::from_millis(500));
            let response = Response::Choice('y');
            return Some(Observation::new(self.clone(), response))
        }
    }

    #[derive(Clone, PartialEq)]
    pub struct Observation {
        pub trial: Trial,
        pub response: Response,
    }

    /// An observation is composed of a trial and an observation

    // We will need access to higher level information
    // to add part and exp level data

    impl Observation {
        pub fn new(trial: Trial, response: Response) -> Self {
            Self{trial: trial, response: response}
        }
    }

    use image;
    #[derive(Clone, PartialEq)]
    pub enum Stimulus {
        Blank(Duration),
        Text(Duration, i8, [i8; 3]),
        Image(Duration, image::RgbaImage, [usize; 4]),
    }

    impl Stimulus{
        pub fn load(&mut self) -> &Self
        {self}
    }

    #[derive(Clone, PartialEq)]
    pub enum Prelude {
        Now,
        Blank(Duration),
        Fix(Duration),
        Prime(Duration, Stimulus),
    }

    #[derive(Clone, PartialEq)]
    pub enum Advance {
        Wait(Duration),
        Keys(Vec<Key>),
        KeysMaxWait(Vec<Key>, Duration)
    }

    #[derive(Clone, Copy, PartialEq)]
    pub enum Response {
        RT(Duration),
        RTCorrect(Duration, bool),
        Choice(Key),
        Graded(f32),
        TooLate,
    }

    #[derive(Clone, Copy, PartialEq)]
    pub enum Feedback{Correct, Incorrect, ThankYou}
}



/// Output
/// 
/// in terms of
/// + event stream
/// + observations

pub mod output {
    use super::{Key, Duration};
    use super::session::Participant;
    use super::trial::{Stimulus, Response};

    #[allow(dead_code)]
    enum YexError {
        FileNotFound(Stimulus),
        PartInterrupt(Participant),

    }

    #[allow(dead_code)]
    enum YldEvent {
        Error(YexError),
        Block(usize),
        Relax(Duration),
        FixCross(Duration),
        StimPresented(Stimulus),
        KeyPress(Key),
        Response(Response),
    }


     
    /*struct YldRecord {
        time: Instant,
        event: YldEvent
    }


    impl YldEvent {
        fn to_csv(self) ->  String {
            format!("{},{}", "time", "event")
        }
    }*/

}
