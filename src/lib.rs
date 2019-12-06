use ggez::{Context, GameResult, nalgebra as na};
use ggez::graphics::{self, MeshBuilder, Drawable, DrawParam, Rect, BlendMode};

pub struct Perspective {
    x: f32,
    y: f32,
    center: (f32, f32),
    width: f32,
    height: f32,
    scale: f32,
}

pub struct Position {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
    pub scale: f32,
}

impl Perspective {
    pub fn new(width: f32, height: f32) -> Perspective {
        Perspective {
            x: 0.0,
            y: 0.0,
            width,
            height,
            center: (width / 2.0, height),
            scale: 1.0,
        }
    }
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
    pub fn set_vanishing_point(&mut self, x: f32, y: f32) -> &mut Self {
        self.center = (x, y);
        self
    }
    pub fn set_scale(&mut self, scale: f32) -> &mut Self {
        self.scale = scale;
        self
    }
    pub fn vanishing_point(&self) -> (f32, f32) {
        let (x, y) = self.center;
        (x, y)
    }
    pub fn dimensions_at_depth(&self, level: f32) -> Position {
        let mut depth = level;
        let base = 1.0;
        if depth == 0.0 {
            depth = 0.01;
        }
        let scale = | distance | distance / (depth * self.scale);
        let position_x = | position | position + self.x;
        let position_y = | position | position + self.y;
        let (x, y) = self.vanishing_point();

        let distance_to_left = x;
        let distance_to_top = y;
        let distance_to_bottom = self.height - y;
        let distance_to_right = self.width - x;

        let top = y - scale(distance_to_top);
        let left = x - scale(distance_to_left);
        let bottom = y + scale(distance_to_bottom);
        let right = x + scale(distance_to_right);

        Position {
            bottom: position_y(bottom),
            right: position_x(right),
            top: position_y(top),
            left: position_x(left),
            scale: scale(base),
        }
    }
}


impl Drawable for Perspective {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let line_width = 1.0;
        let point = param.dest;
        let mesh: &mut MeshBuilder = &mut MeshBuilder::new();
        let color = graphics::WHITE;

        for depth in 0..10 {
            let Position {
                top,
                left,
                bottom,
                right,
                ..
            } = self.dimensions_at_depth(depth as f32);

            let top_left_to_top_right = [
                na::Point2::new(left, top),
                na::Point2::new(right, top),
            ];

            let top_right_to_bottom_right = [
                na::Point2::new(right, top),
                na::Point2::new(right, bottom),
            ];

            let bottom_left_to_bottom_right = [
                na::Point2::new(left, bottom),
                na::Point2::new(right, bottom),
            ];

            let bottom_left_to_top_left = [
                na::Point2::new(left, top),
                na::Point2::new(left, bottom),
            ];

            mesh.line(&top_left_to_top_right, line_width, color)?;
            mesh.line(&top_right_to_bottom_right, line_width, color)?;
            mesh.line(&bottom_left_to_bottom_right, line_width, color)?;
            mesh.line(&bottom_left_to_top_left, line_width, color)?;
        }
        let world = mesh.build(ctx)?;

        graphics::draw(ctx, &world,  (point, 0.0, graphics::WHITE))?;
        Ok(())
    }
    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(Rect { w: self.width, h: self.width, x: 0.0, y: 0.0 })
    }
    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {}
    fn blend_mode(&self) -> Option<BlendMode>{
        None
    }
}