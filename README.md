# Yex is Yourself Experiments

The *goal* is to create a simple way of running experiments that presents visual stimuli.
In fact, Yex will be not much more than a basic slideshow program, as it is meant to connect with 
[Ystudio](../ysudio-zero) a [YLab Edge](../ylab-edge-go).

Yex is written in Rust, which makes it easy create executables for many platforms, even web and mobile devices.

Planned features:
- timed presentation
- collecting a variety of response types (RT, correctness, choice)
- image stimuli
- stimulus preludes (e.g. fixation cross)
- key press events
- event recording

## Technical design

1. The data hub in the Y-verse is [Ystudio](../ytsudio-zero). In the first place, Yex will not be controlled by Ystudio. It runs on its own, just like another sensor and reports event data to Ystudio, where it is collected and merged with YLab data. That will also make it easy to run it on two screens, or even two computers. 
1. The yeta_1 program of [Yet](../yet) can act as a model. Like with Yet, the user has to specify their stimuli as CSV files (stimuli, AOI)
1. architecture follws the Model-View-Controller pattern. 
    +  Yex *Engine* prepares a session and runs through it
        + using timers
        + handling input events from *UI*
        + updating or sending data to other actors
        + collecting and time-staming events
    +  Yex *UI* receives states and assests from *Engine* and renders the interface (can be called a REST approach)
    +  *YexRecord* defines an extensible protocol for communicating updates and sending assets (e.g. stimuli) over channels
    +  Most of the *Data* components are static, heavily leaning on enums ontologies and matching in Rust. Yex *Trans* is the data sink and is responsible for serializing and sending out events and data from *Engine* (and possibly also UI) and transport them to places that are safe or more interesting, like [Ystudio](../ystudio-zero).
1. *architecture* is based on actors
    +  Engine, UI and Storage are agnostic actors, running in parallel threads, keeping their own data and timing. 
    +  Actors communicate via *channels* (uni-directional multi-producer-single-consumer). For example, UI opens a channel and listens on it. Engine takes the sender component. Whenever Engine gets into a new state it composes a message to UI, and encapsulates all the information UI needs to render the interface, e.g. an image.
    +  A complex protocol *YexRecord* is used to communicate effectively over a few channels.
    +  Storage receives event data, filters and serializes it and sends it to the filesystem or [Ystudio]()
1. *Ontology* (make heavy use of Rusts enums, tuple structs and matching):
    +  *Engine* is the central actor, which initiates a *Session* data object
    +  *Sessions* are encounters of a participants with an *Experiments*
    +  *Experiments* are ordered sets of *Trials*, 
    +  which can be partitioned into *Blocks*,
    +  Blocks can be in three states: Prelude, Trials, Relax
    +  Trials can be in three states: Prelude, Present, Feedback,
1. Processes and Concurrency
    +  parallel threads for Engine, Trans and UI
    +  on Engine level, a Session is strictly ordered, allowing us to use plain sequential programming
    +  async/await concurrency should only be used locally, where firing up a new thread is not worth the effort,  e.g. keypress with timeout.



