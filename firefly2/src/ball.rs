use nannou::prelude::*;

pub struct Ball {
    lightness: f32,
    radius: f32,
    distance: f32,
    theta: f32,
    theta_offset: f32,
}

impl Ball {
    pub fn new(lightness: f32, radius: f32, distance: f32, theta: f32, theta_offset: f32) -> Ball {
        Ball {
            lightness,
            radius,
            distance,
            theta,
            theta_offset,
        }
    }

    pub fn draw(&self, center: &Point2, draw: &Draw) {
        let theta = self.theta + self.theta_offset;
        let point = pt2(theta.cos(), theta.sin()) * self.distance;
        draw.ellipse()
            .color(hsla(0.0, 0.0, self.lightness, 0.4))
            .w_h(self.radius, self.radius)
            .x_y(center.x + point.x, center.y + point.y);
    }

    pub fn set_theta(&mut self, theta: f32) {
        self.theta = theta;
    }

    pub fn get_theta(&self) -> f32 {
        self.theta
    }
}
