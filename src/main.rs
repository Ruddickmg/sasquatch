use ggez;
use ggez::event;
use ggez::{Context, GameResult, nalgebra as na};
use ggez::graphics::{self, Rect};
use sasquatch;

struct MainState {
    inc_x: f32,
    inc_h: f32,
    pos_x: f32,
    pos_z: f32,
    horizon: f32,
}

impl MainState {
    fn new(horizon: f32) -> ggez::GameResult<MainState> {
        let s = MainState { pos_x: 0.0, horizon, pos_z: 1.0, inc_h: 1.0, inc_x: 1.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> ggez::GameResult {
        if self.pos_x <= 0.0 {
            self.inc_x = 1.0;
        } else if self.pos_x >= 800.0 {
            self.inc_x = -1.0;
        }
        if self.horizon <= 0.0 {
            self.inc_h = 1.0;
        } else if self.horizon >= 600.0 {
            self.inc_h = -1.0;
        }
        self.pos_x = self.pos_x + self.inc_x;
        self.horizon = self.horizon + (self.inc_h * 0.3);
        self.pos_z = self.pos_z % 100.0 + 0.1;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let params = (na::Point2::new(0.0, 0.0),);
        let Rect { w, h, .. } = graphics::screen_coordinates(ctx);
        let mut grid_world = sasquatch::Perspective::new(w, h);

        grid_world.set_vanishing_point(self.pos_x, self.horizon);

        let dimensions = grid_world.dimensions_at_depth(self.pos_z);
        let radius = 300.0 * dimensions.scale;
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(dimensions.left + ((dimensions.right - dimensions.left) / 2.0), dimensions.top + ((dimensions.bottom - dimensions.top) / 2.0)),
            radius,
            2.0,
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &circle, params)?;
        graphics::draw(ctx, &grid_world, params)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Sasquatch", "Marcus Ruddick");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(600.0)?;
    event::run(ctx, event_loop, state)
}
