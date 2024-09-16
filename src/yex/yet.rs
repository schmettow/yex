use std::sync::mpsc::{Receiver, Sender};

pub mod yet {
    use nannou::color::Srgb;
    use super::*;

    type YetCtr = Receiver<YetCmd>;
    enum YetCmd {NewPoint((f32, f32)), Train, QuickCalibrate((f32, f32))}
    enum YetRes {EyePos((f32, f32))}
    type YetOut = Sender<YetRes>;
    #[derive(Debug, Clone)]
    pub struct Calibration {
        pub state: State,
        pub points: Vec<(f32, f32)>,
        pub radius: f32,
        pub color: Srgb<u8>,
        pub active_color: Srgb<u8>,
        pub bg_color: Srgb<u8>,
    }

    #[derive(Debug, Clone)]
    pub enum State {
        Prelude,
        Target(usize),
    }

    impl Default for Calibration {
        fn default() -> Self {
            Self {
                state: State::Prelude,
                points: vec![(-0.25, -0.25),(0.25, -0.25),(0.25, 0.25),(-0.25, 0.25)],
                radius: 0.05,
                color: Default::default(),
                active_color: Default::default(),
                bg_color: Default::default(),
            }
        }
    }

    type MLModel = [f32; 4]; // quad bright
    pub struct Yet {
        model: MLModel,
        offset: ((f32, f32)),
    }

    impl Yet {
        pub fn train(&mut self) -> () {}
        pub fn run(&mut self) -> () {
            loop{

            }
        }

    }
}
