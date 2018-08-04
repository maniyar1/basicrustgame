
extern crate ggez;
use ggez::*;
use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{DrawMode, Point2};
use ggez::conf::FullscreenType;

// Holy fucking shit I just wrote this and I don't know what it doe
struct MainState {
    pos_x: f32,
    pos_y: f32,
    sprite: graphics::Image,
    speed: f32,
    up: bool,
    down: bool,
    left: bool,
    right: bool
}
impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0, pos_y: 380.0, sprite:graphics::Image::new(_ctx, "/mainplayer.png").unwrap(), speed: 20.0, up: false, down: false, left: false, right: false };
        Ok(s)
    }
}

impl event::EventHandler for MainState {

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool)    {
    println!(
        "Key pressed: {:?}, modifier {:?}, repeat: {}",
        keycode,keymod, repeat
    );
            match keycode {
               Keycode::Up => {
                   println!("UP!");
                   self.up = true;
               }
               Keycode::Left => {
                   self.left = true;
               }
               Keycode::Right => {
                   self.right = true;
               }
               Keycode::Down => {
                   self.down = true;
               }
               Keycode::Escape => _ctx.quit().unwrap(),
               _ => (), // Do nothing
           }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool)    {
    println!(
        "Key pressed: {:?}, modifier {:?}, repeat: {}",
        keycode,keymod, repeat
    );
            match keycode {
               Keycode::Up => {
                   println!("UP!");
                   self.up = false;
               }
               Keycode::Left => {
                   self.left = false;
               }
               Keycode::Right => {
                   self.right = false;
               }
               Keycode::Down => {
                   self.down = false;
               }
               Keycode::Escape => _ctx.quit().unwrap(),
               _ => (), // Do nothing
           }
    }
    // Guys turn back, don't even try
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.right {
         self.pos_x = self.pos_x + self.speed;
        } else if self.down {
              self.pos_y = self.pos_y + self.speed;
        } else if self.up {
              self.pos_y = self.pos_y - self.speed;
        } else if self.left {
              self.pos_x = self.pos_x - self.speed;
        } else {
              self.pos_x = self.pos_x;
              self.pos_y = self.pos_y;
      }
      println!("pos_x {}, pos_y {}", self.pos_x, self.pos_y);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        ggez::graphics::set_resolution(ctx, 1280, 720);
        let rect = ggez::graphics::Rect::new(0.0, 0.0, 1280.0, 720.0);
        ggez::graphics::set_screen_coordinates(ctx, rect).unwrap();
        graphics::set_background_color(ctx, graphics::WHITE);
        let point = graphics::Point2::new(self.pos_x, self.pos_y);
        graphics::clear(ctx);
        graphics::draw(ctx, &self.sprite, point, 0.0);
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_mode.dimensions(1280, 720);
    c.window_mode.max_dimensions(1280, 720);
    c.window_setup.resizable = true;
    let ctx = &mut Context::load_from_conf("simple", "me", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
