use crate::ball::Ball;
use crate::ballgroup::BallGroup;
use crate::ballgroup::RotateDirection;
use nannou::prelude::*;

pub mod ball;
pub mod ballgroup;

struct Model {
    ballgroups: Vec<BallGroup>,
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    let mut ballgroups = Vec::new();

    let mut ballgroup = BallGroup::new(
        pt2(-200.0, 0.0),
        RotateDirection::CLOCKWISE,
        random_f32() * 2.0 * PI,
    );
    for _ in 0..200 {
        let lightness = random_f32() + 0.1;
        let distance = random_f32() * 250.0 + 100.0;
        let theta_offset = random_f32() * 2.0 * PI;
        let ball = Ball::new(lightness, 10.0, distance, 0.0, theta_offset);
        ballgroup.add_ball(ball);
    }
    ballgroups.push(ballgroup);

    let mut ballgroup = BallGroup::new(
        pt2(200.0, 0.0),
        RotateDirection::COUNTERCLOCKWISE,
        random_f32() * 2.0 * PI,
    );
    for _ in 0..200 {
        let lightness = random_f32() + 0.1;
        let distance = random_f32() * 250.0 + 100.0;
        let theta_offset = random_f32() * 2.0 * PI;
        let ball = Ball::new(lightness, 10.0, distance, 0.0, theta_offset);
        ballgroup.add_ball(ball)
    }
    ballgroups.push(ballgroup);
    Model { ballgroups }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn update(app: &App, model: &mut Model, _update: Update) {
    let delta = 0.01;

    for ballgroup in &mut model.ballgroups {
        ballgroup.rotate(delta);
        ballgroup.move_center(app.main_window().inner_size_points());
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.blend(BLEND_ADD);

    // to draw shadow
    let window = app.window_rect();
    draw.rect().wh(window.wh()).rgba(0.0, 0.0, 0.0, 0.4);
    draw.to_frame(app, &frame).unwrap();

    // draw balls
    for ballgroup in &model.ballgroups {
        ballgroup.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
