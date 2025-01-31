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

    /// Default Block
    ///
    /// Three trials in given order with
    /// + default trial
    /// + a 1s blank prelude
    /// + 2s relaxation period
    ///
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
    use std::path::PathBuf;
    use std::time::Instant;
    use crate::output::{YexError, YexRecord};

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

    /// Default Trial
    ///
    /// Three trials in given order with
    /// + 500ms each for prelude, stim and time advance
    /// +
    /// + a 1s blank prelude
    /// + 2s relaxation period
    ///
    impl Default for Trial {
        fn default() -> Self {
            Self {  state: State::Init,
                prelude: Prelude::Blank(Duration::from_micros(500)) ,
                stimulus: Stimulus::Blank{dur: Duration::from_micros(500)},
                advance: Advance::Wait(Duration::from_millis(500))}
        }
    }

    impl Trial {

        pub fn prepare(&mut self) -> Result<(), YexError> {
            let mut stimulus = &self.stimulus;
            match stimulus {
                Stimulus::Image {path, dur, mut img} => {
                    stimulus.load();
                    Ok(())
                },
                _ => Err(YexError::FileNotFound)
            }

        }
        pub fn run(&mut self, events_out: Sender<YexRecord>) -> Option<Observation> {
            events_out.send(YexEvent::Trial(self.state.clone()).into()).unwrap();
            self.prepare().unwrap();
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
            let time_when_presented = Instant::now();
            // Emulating the incoming response from the participant.
            //
            // Here we will have time-outs and user events intermixed.
            // Would be nice to have some async here, maybe
            // block_on(select())
            sleep(Duration::from_millis(500));
            let rt = time_when_presented.elapsed();
            let response = Response::Choice(rt, 'y');
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



    /// Observations
    ///
    /// ... are the combination of a stimulus and a response.
    /// The main purpose is to create rows for a data frame.

    /// Note that this does not provide access to participant data.

    #[derive(Clone, PartialEq)]
    pub struct Observation {
        pub trial: Trial,
        pub response: Response,
    }

    impl Observation {
        const OBS_TBL_HEAD: DF = df!(
                "presentation_time" => vec!([] as f64),
                "rt" => vec!([] as f64),
            ).unwrap();

        pub fn new(trial: Trial, response: Response) -> Self {
            Self{trial, response}
        }

        pub fn get_row(&self) -> DF {
            let &stimulus = &self.trial.stimulus;
            let &response = &self.response;
            let presentation_time: f64 = stimulus.get_ptime().as_millis() as f64;
            let presentation_time = Series::new("ptime".into(),[presentation_time]);
            let rt: Option<f64> = Some(response.rt().unwrap().as_millis() as f64);
            let rt = Series::new("ptime".into(), [rt]);
            DF::new(vec!(presentation_time, rt)).unwrap()

        }

    }

    use polars::*;
    use polars::frame::DataFrame as DF;
    use polars::series::Series;
    impl Into<DF> for Observation {
        fn into(self) -> DF {
            let stimulus = self.trial.stimulus;
            let response = self.response;
            let presentation_time: f64 = stimulus.get_ptime().as_millis() as f64;
            let presentation_time = Series::new("ptime".into(),[presentation_time]);
            let rt: Option<f64> = Some(response.rt().unwrap().as_millis() as f64);
            let rt = Series::new("ptime".into(), [rt]);
            DF::new(vec!(presentation_time, rt)).unwrap()
        }
    }

    pub fn to_dataframe(mut obs: Vec<Observation>) -> DF {
        let mut out = df!(
                "presentation_time" => vec!([] as f64),
                "rt" => vec!([] as f64),
            ).unwrap();
        for o in obs[1..].iter() {
            out.extend(o.data_row());
        }
        out
    }

    impl Observation {
        fn into(self) -> DF {
            let stimulus = self.trial.stimulus;
            let response = self.response;
            let presentation_time: f64 = stimulus.get_ptime().into();
            let rt: Option<f64> = response.rt().into();
            df!(
                "presentation_time" => [presentation_time],
                "rt" => [rt],
            );

        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Response {
        RT(Duration),
        Choice(Duration, Key),
        Graded(Duration, f32),
        TimedTest(Duration, bool),
        TooLate(),
    }

    impl Response {

        /// Reaction time
        ///
        /// Extracts some reaction time, or None
        pub fn rt(self) -> Option<Duration> {
            match self {
                Response::RT(dur) => Some(dur),
                Response::Choice(dur, _) => Some(dur),
                Response::Graded(dur, _) => Some(dur),
                Response::TimedTest(dur, result) => Some(dur),
                Response::TooLate() => None,
            }
        }
    }



    use image;
    use image::RgbaImage;
    use polars::chunked_array::ChunkedArray;
    use polars::prelude::{NamedFrom, PlSmallStr};

    /*pub struct StimText {
        pub dur: Duration,
        pub text: String,
    }

    pub struct StimImage {
        pub dur: Duration,
        pub path: PathBuf,
        pub img : Option<image::RgbaImage>,
    }

    impl Stimulus::Image {
        pub fn load(&mut self) -> &Self {
            let raw_img = image::open(self.path()).unwrap();
            self.img = Some(raw_img.into());
            }
    }

    pub struct StimBlank {
        pub dur: Duration,
    }
    */

    #[derive(Clone, Debug, PartialEq)]
    pub enum Stimulus {
        Blank { dur: Duration },
        Text  { dur: Duration,
                text: String,},
        Image { dur: Duration,
                path: PathBuf,
                img : Option<RgbaImage>,},
    }

    impl Stimulus {
        pub fn get_ptime(self) -> Duration {
            match self {
                Stimulus::Blank { dur } => dur,
                Stimulus::Text { dur, .. } => dur,
                Stimulus::Image { dur, .. } => dur,
            }
        }

        pub fn load(&mut self) -> Option<()> {
            if let Stimulus::Image{dur, path, mut img} = self {
                if let raw_img = image::open(self.path()).unwrap() {
                    img = Some(raw_img.into());
                    Some(())
                }
            }
            None
        }

        pub fn get_path(self) -> Option<PathBuf> {
            if let Stimulus::Image{dur, path, img} = self {
                return Some(path)
            }
            None
        }

        pub fn id(&self) -> PlSmallStr {
            if let Stimulus::Image{dur, path, img} = self {
                return Some(path().pop().into())
            }
            None
        }


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

    #[derive(Clone, Copy, PartialEq)]
    pub enum Feedback{Correct, Incorrect, ThankYou}
}

