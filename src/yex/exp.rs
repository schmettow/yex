/// Building sessions
///
/// A session is the encounter of a participant with an experiment.
///
/// + composed of a Participant and Experiment object.
/// + runs linearly through the steps of the experiment
/// + sending high-level events


use super::*;
pub mod session {
    use super::*;
    use super::block::Block;

    #[derive(Debug, Clone)]
    pub struct Session {
        pub id: Instant,
        pub part: Participant,
        pub exp: Experiment,
        pub state: State,
    }

    #[derive(Debug, Clone)]
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
            Session{id: Instant::now(), part, exp, state: State::Init}
        }
    }

    impl Default for Session {
        fn default() -> Self {
            Self::new(Experiment::default(), Participant::default())
        }
    }

    #[derive(Clone, Debug)]
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

    #[derive(Clone, Debug)]
    pub enum Sex {
        Male,
        Female,
    }

    #[derive(Clone, Debug)]
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


    #[derive(Clone, Debug)]
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
    use super::{Sender, Duration, Instant, sleep, Key, Text, YexRecord, YexEvent};

    /// A Block is a sequences of Trials
    ///
    /// with a prelude and relax frame.
    ///
    /// + running through trials
    /// + sending block-level events
    ///
    #[derive(Clone, Debug)]
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
                trials,
                random: false,
                prelude: Prelude::Blank(Duration::from_millis(1000)),
                relax: Relax::Wait(Duration::from_millis(2000)),
                state: State::Init,
            }
        }
    }

    #[derive(Clone, PartialEq, Debug)]
    /// Block states
    ///
    pub enum State {
        Init,
        Prelude(Prelude),
        Trials(), // trial number
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
    #[derive(Clone, Debug)]
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
        pub fn run(&mut self, events_out: Sender<YexRecord>) -> Option<Vec<Observation>> {
            events_out.send(YexEvent::Block(self.state.clone()).into()).unwrap();
            let mut out: Vec<Observation> = Vec::new();
            self.state = State::Prelude(self.prelude.clone());
            events_out.send(YexEvent::Block(self.state.clone()).into()).unwrap();
            match self.prelude.clone() {
                Prelude::Now
                => {},
                Prelude::Blank(dur)
                => {sleep(dur)},
                Prelude::Instruct(dur, _)
                => {sleep(dur);},
                _   => todo!(),
            }
            self.state = State::Trials();
            events_out.send(YexEvent::Block(self.state.clone()).into()).unwrap();
            for trial in self.trials.clone(){
                // making an observation by running a trial
                let obs
                    = trial.clone().run(events_out.clone());
                match obs {
                    None => {},
                    Some(obs) => {
                        // collecting new observation
                        out.push(obs);}
                }
            }

            self.state = State::Relax;
            events_out.send(YexEvent::Block(self.state.clone()).into()).unwrap();
            match self.relax {
                Relax::Now
                => {}, // do nothing is not the same as not implemented
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
    use crate::output::YexRecord;

    use super::{Duration, sleep, Key, Sender, YexEvent};

    /// A trial is a Stimulus with a Prelude and Advance frame
    ///

    #[derive(Clone, Debug, PartialEq)]
    pub struct Trial {
        pub prelude: Prelude,
        pub stimulus: Stimulus,
        pub advance: Advance,
        pub state: State
    }

    #[derive(Clone, PartialEq, Debug)]
    pub enum State {
        Init,
        Prelude,
        Present(Stimulus),
        Feedback()
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
        pub fn run(&mut self, events_out: Sender<YexRecord>) -> Option<Observation> {
            events_out.send(YexEvent::Trial(self.state.clone()).into()).unwrap();
            self.prepare();
            self.state = State::Prelude;
            events_out.send(YexEvent::Trial(self.state.clone()).into()).unwrap();
            match self.prelude {
                Prelude::Now => {},
                Prelude::Blank(dur) | Prelude::Fix(dur)
                => {sleep(dur);},
                Prelude::Prime(_,_) => todo!(),
            }
            self.state = State::Present(self.stimulus.clone());
            events_out.send(YexEvent::Trial(self.state.clone()).into()).unwrap();
            // Emulating the incoming response from the participant.
            //
            // Here we will have time-outs and user events intermixed.
            // Would be nice to have some async here, maybe
            // block_on(select())
            sleep(Duration::from_millis(500));
            let response = Response::Choice('y');
            events_out.send(YexEvent::Response(response).into()).unwrap();
            self.state = State::Feedback();
            events_out.send(YexEvent::Trial(self.state.clone()).into()).unwrap();
            match self.advance {
                Advance::Wait(dur) => {sleep(dur)},
                _ => {todo!();},
            }
            Some(Observation::new(self.clone(), response))
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
            Self{trial, response}
        }
    }

    use image;
    #[derive(Clone, Debug, PartialEq)]
    pub enum Stimulus {
        Blank(Duration),
        Text(Duration, i8, [i8; 3]),
        Image(Duration, image::RgbaImage, [usize; 4]),
    }

    impl Stimulus{
        pub fn load(&mut self) -> &Self
        {self}
    }

    #[derive(Clone, PartialEq, Debug)]
    pub enum Prelude {
        Now,
        Blank(Duration),
        Fix(Duration),
        Prime(Duration, Stimulus),
    }

    #[derive(Clone, PartialEq, Debug)]
    pub enum Advance {
        Wait(Duration),
        Keys(Vec<Key>),
        KeysMaxWait(Vec<Key>, Duration)
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
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

