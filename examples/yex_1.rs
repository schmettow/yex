use std::ops::{Div, Mul};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use image::{DynamicImage, RgbaImage};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use yex::output::YexRecord;
use yex::session::{Experiment, Participant, Session};
use yex::yet;
use crate::State::Calibration;

/// Maximum # targets per row/col
const MAX_ROOT_TARGETS: usize = 16;
const WIDTH: usize = 1000;
const HEIGHT: usize = 800;
const BG_COLOR: Srgb<u8> = BLACK;


fn main() {
    nannou::app(model).update(update).run();
}



/*enum Settings {
    Yet {
        targets: Vec<Vec2>,
        radius: f32,
        color: Srgb<u8>,
        active_color: Srgb<u8>,
        },
    Experiment {},
}*/


enum State {Setup, Calibration(yex::yet::Calibration), Experiment(yex::Session)}

struct Model {
    state: State,
    egui: Egui,
}

fn model(app: &App) -> Model {
    let exp = Experiment::default();
    println!("Exp with {} blocks", exp.blocks.len());
    // Dummy participant
    let part = Participant::default();
    println!("Hello {}.", part.id);
    // Creating the session
    let session = Arc::new(Mutex::new(Session::new(exp, part)));
    // Starting the event recorder channel
    let (event_snd, event_rec)  = channel::<YexRecord>();
    // Detached mock task to receive and print the events
    // Create window
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let assets = app.assets_path().unwrap();
    let img_dir = assets.join("Stimuli");
    let mut stimuli: Vec<RgbaImage> = vec![];
    for entry in std::fs::read_dir(&img_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let image = match image::open(&path) {
            Ok(img) => {
                img.into_rgba8()},
            Err(err) => {
                eprintln!("failed to open {} as an image: {}", path.display(), err);
                continue;
            }
        };
        stimuli.push(image);

    }

    Model {
        state: State::Setup,
        egui,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    //let settings = &mut model;
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    match &model.state {
        State::Setup => {
            egui::Window::new("Settings")
                .show(&ctx, |ui| {
                    if ui.button("Start Calibration").clicked() {
                        let yet = yex::yet::Yet::new();
                        model.state = Calibration(yex::yet::Calibration::default());
                    }
                });},
        State::Calibration(mut calib) => {
            egui::Window::new("Calibration")
                .show(&ctx, |ui| {
                    match mut calib.state {
                        yet::State::Prelude => calib.state = yet::State::Prelude,

                    }
                });
        },
        State::Experiment(mut session) => {
            egui::Window::new("Settings")
                .show(&ctx, |ui| {
                    if ui.button("Start Calibration").clicked() {
                        let yet = yex::yet::Yet::new();
                        model.state = Calibration(yex::yet::Calibration::default());

                    }
                });
        },
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}


fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;
    let dim = frame.rect().wh();
    let draw = app.draw();
    draw.background().color(BG_COLOR);
    match &model.state {
        State::Calibration(active) => {
            for (i,t) in model.settings.targets.iter().enumerate() {
                let abs_pos = t.mul(dim).div(2.0);
                draw_target(&draw, &abs_pos, false, &settings);
        }},
        State::Stimulus(stim_no) => {
            let stim = model.stimuli[stim_no];
        },
        State::Setup => {}
        }
    //println!("Imagine {} dots", settings.n_targets);
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
    }




fn draw_target(draw: &Draw, pos: &Vec2, active: bool, settings: &Calibration) {
    let color: Srgb<u8>;
    if active {
        color = settings.active_color;
    } else {
        color = settings.color;
    }
    draw.ellipse()
        .resolution(100.0)
        .xy(*pos)
        .radius(settings.radius)
        .rotate(0.0)
        .color(color)
    ;
    draw.ellipse()
        .resolution(100.0)
        .xy(*pos)
        .radius(settings.radius * 0.95)
        .rotate(0.0)
        .color(BG_COLOR)
    ;
}


fn draw_calib_screen(draw: &Draw, &calib: Calibration){

},