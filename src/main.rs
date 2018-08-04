extern crate ggez;
use ggez::event::{self, Keycode, Mod};
use ggez::*;

// Holy fucking shit I just wrote this and I don't know what it does
//TODO Make variable names not awful
struct MainState {
    pos_x: f32,
    pos_y: f32,
    hitbox_size: f32, // Assumes hitbox is square, as so it is width (or length) of hitbox divided by two
    combat_mode: bool,
    sprite: graphics::Image,
    speed: f32,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    enemy_x: f32,
    enemy_y: f32,
    enemy_sprite: graphics::Image,
    grass_background: graphics::Image,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            pos_x: 0.0,
            pos_y: 380.0,
            hitbox_size: 110.0,
            combat_mode: false,
            sprite: graphics::Image::new(_ctx, "/mainplayer.png").unwrap(),
            speed: 20.0,
            up: false,
            down: false,
            left: false,
            right: false,
            enemy_x: 600.0,
            enemy_y: 380.0,
            enemy_sprite: graphics::Image::new(_ctx, "/enemy1.png").unwrap(),
            grass_background: graphics::Image::new(_ctx, "/grass_background.png").unwrap(),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn resize_event(&mut self, ctx: &mut Context, _width: u32, _height: u32) {
        let rect = ggez::graphics::Rect::new(0.0, 0.0, 1920.0, 1080.0);
        ggez::graphics::set_screen_coordinates(ctx, rect).unwrap();
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
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
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
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
        let hitboxdiffx = self.pos_x - self.enemy_x;
        let hitboxdiffy = self.pos_y - self.enemy_y;

        let abshitboxdiffx = hitboxdiffx.abs();
        let abshitboxdiffy = hitboxdiffy.abs();
        // println!("hitboxdiffx {}, hitboxdiffy {}, abshitboxdiffx {}, abshitboxdiffy{}", hitboxdiffx, hitboxdiffy, abshitboxdiffx, abshitboxdiffy);

        if abshitboxdiffx < (self.hitbox_size) && abshitboxdiffy < (self.hitbox_size) {
            // println!("Combat!");
            self.combat_mode = true;
        } else if self.right {
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
        //println!("pos_x {}, pos_y {}", self.pos_x, self.pos_y);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.resize_event(ctx, 1280, 720);
        if !self.combat_mode {
            graphics::set_background_color(ctx, graphics::WHITE);
        } else {
            graphics::set_background_color(ctx, graphics::WHITE);
        }
        let backgroundpoint = graphics::Point2::new(0.0, 0.0);
        let point = graphics::Point2::new(self.pos_x, self.pos_y);
        let enemy_point = graphics::Point2::new(self.enemy_x, self.enemy_y);

        graphics::clear(ctx);
        if !self.combat_mode {
            graphics::draw(ctx, &self.grass_background, backgroundpoint, 0.0).unwrap();
            graphics::draw(ctx, &self.enemy_sprite, enemy_point, 0.0).unwrap();
            graphics::draw(ctx, &self.sprite, point, 0.0).unwrap();
        } else {

        }
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_mode.dimensions(1920, 1080);
    c.window_mode.max_dimensions(1920, 1080);
    c.window_setup.resizable = true;

    let ctx = &mut Context::load_from_conf("simple", "me", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
