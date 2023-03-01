use crate::ball::Ball;
use nannou::prelude::*;

pub enum RotateDirection {
    CLOCKWISE,
    COUNTERCLOCKWISE,
}

pub struct BallGroup {
    center: Point2,
    balls: Vec<Ball>,
    rotate_direction: RotateDirection,
    move_direction: f32,
}

impl BallGroup {
    pub fn new(
        center: Point2,
        rotate_direction: RotateDirection,
        move_direction: f32,
    ) -> BallGroup {
        let balls = Vec::<Ball>::new();
        BallGroup {
            center,
            balls,
            rotate_direction,
            move_direction,
        }
    }

    pub fn add_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    pub fn rotate(&mut self, delta: f32) {
        let d = match self.rotate_direction {
            RotateDirection::CLOCKWISE => -delta,
            RotateDirection::COUNTERCLOCKWISE => delta,
        };
        for ball in &mut self.balls {
            ball.set_theta(ball.get_theta() + d)
        }
    }

    pub fn move_center(&mut self, window_size: (f32, f32)) {
        let delta = pt2(self.move_direction.cos(), self.move_direction.sin()) * 4.0;
        self.center += delta;
        if self.center.x > window_size.0 / 2.0 || self.center.x < -window_size.0 / 2.0 {
            self.move_direction += (PI / 2.0 - self.move_direction) * 2.0;
        }
        if self.center.y > window_size.1 / 2.0 || self.center.y < -window_size.1 / 2.0 {
            self.move_direction *= -1.0
        }
        if self.move_direction > 2.0 * PI {
            self.move_direction -= 2.0 * PI
        }
        if self.move_direction < 0.0 {
            self.move_direction += 2.0 * PI
        }
    }

    pub fn draw(&self, draw: &Draw) {
        for ball in &self.balls {
            ball.draw(&self.center, draw);
        }
    }

    pub fn get_center(&self) -> Point2 {
        self.center
    }

    pub fn set_center(&mut self, center: Point2) {
        self.center = center;
    }
}
