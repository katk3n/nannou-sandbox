use crate::ball::Ball;
use nannou::prelude::*;

pub mod ball;

struct Model {
    balls: Vec<Ball>,
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    let mut balls = Vec::new();
    for _ in 0..150 {
        let hue = random_f32();
        let distance = random_f32() * 250.0 + 100.0;
        let theta_offset = random_f32() * 2.0 * PI;
        let ball = Ball::new(hue, 20.0, 0.0, 0.0, distance, 0.0, theta_offset);
        balls.push(ball);
    }
    Model { balls }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = app.time;
    let theta = 0.5 * t as f32;

    for ball in &mut model.balls {
        ball.set_theta(theta);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.blend(BLEND_ADD);
    draw.background().color(BLACK);

    for ball in &model.balls {
        ball.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
