use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;

fn main() {
    nannou::app(model).run();
}

struct Model {
    stream: audio::Stream<Audio>,
}

struct Audio {
    sound: audrey::read::BufFileReader,
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();

    let audio_host = audio::Host::new();
    let assets = app.assets_path().expect("could not find assets directory");
    let path = assets.join("sounds").join("impro10.wav");
    dbg!(&path);
    let sound = audrey::open(path).expect("failed to load sound");
    let model = Audio { sound };
    let stream = audio_host
        .new_output_stream(model)
        .render(audio)
        .build()
        .unwrap();

    stream.play().unwrap();

    Model { stream }
}

fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    let file_frames = audio.sound.frames::<[f32; 2]>().filter_map(Result::ok);
    for (frame, file_frame) in buffer.frames_mut().zip(file_frames) {
        for (sample, file_sample) in frame.iter_mut().zip(&file_frame) {
            *sample += *file_sample;
        }
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(DIMGRAY);
}
