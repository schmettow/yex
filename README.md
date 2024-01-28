# Yex is Your Experiment

The *goal* is to create a simple way of running experiments that presents visual stimuli.
In fact, Yex will be not much more than a basic slideshow program, as it is meant to connect with 
[Ystudio](../ysudio-zero) a [YLab Edge](../ylab-edge-go).

Yex is written in Rust, which makes it easy create executables for many platforms, even web and mobile devices.

Planned features:
- timed presentation
- event-controlled presentation
- pre-stimuli (e.g. fixation cross)
- key press events

## Technical ideas

1. The data hub in the Y-verse is Ystudio. Yex will not be controlled by Ystudio. It runs like another sensor and reports event data to Ystudio, where it is collected and merged with YLab data. That will also make it easy to run it on two screens, or even two computers. 
Will have to look into doing IPC for that matter.
1. The presenter component of [Yet](../yet) can act as a model. Like with Yet, the user has to specify their stimuli as CSV files (stimuli, AOI)

## TODO
* [ ] Live-reloading of `slides.md`

### Running it

`cargo run --release`

### Running the web version
* `rustup target add wasm32-unknown-unknown`
* `cargo install --locked trunk`
* `trunk serve`
* open `http://127.0.0.1:8080/index.html#dev`

### Web Deploy
Should deploy automatically by the CI action.
