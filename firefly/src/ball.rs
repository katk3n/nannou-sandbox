use nannou::prelude::*;

pub struct Ball {
    hue: f32,
    radius: f32,
    x: f32,
    y: f32,
    distance: f32,
    theta: f32,
    theta_offset: f32,
}

impl Ball {
    pub fn new(
        hue: f32,
        radius: f32,
        x: f32,
        y: f32,
        distance: f32,
        theta: f32,
        theta_offset: f32,
    ) -> Ball {
        Ball {
            hue,
            radius,
            x,
            y,
            distance,
            theta,
            theta_offset,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        let num_layer = 5;
        let dr = self.radius / num_layer as f32;
        let theta = self.theta + self.theta_offset;
        let point = pt2(theta.cos(), theta.sin()) * self.distance;
        for i in 1..num_layer {
            let factor: f32 = (num_layer - i) as f32;
            let l = 1.0 - factor * 0.1;
            let r = dr * factor;
            draw.ellipse()
                .color(hsla(self.hue, 1.0, l, 0.4))
                .w_h(r, r)
                .x_y(point.x, point.y);
        }
    }

    pub fn set_theta(&mut self, theta: f32) {
        self.theta = theta;
    }

    pub fn get_theta(&self) -> f32 {
        self.theta
    }

    pub fn set_point(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_point(&self) -> Point2 {
        pt2(self.x, self.y)
    }
}
