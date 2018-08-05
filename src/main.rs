extern crate ggez;
use ggez::event::{self, Keycode, Mod};
use ggez::*;

// Holy fucking shit I just wrote this and I don't know what it does
//TODO Make variable names not awful
struct MainState {
    pos_x: f32,
    pos_y: f32,
    player_health: i32,
    player_noomba: u32,
    hitbox_size: f32, // Assumes hitbox is square, as so it is width (or length) of hitbox divided by two
    combat_mode: bool,
    num1: bool,
    player_turn: bool,
    sprite: graphics::Image,
    speed: f32,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    enemy_x: f32,
    enemy_y: f32,
    enemy_health: i32,
    enemy_noomba: u32,
    enemy_dead: bool,
    enemy_sprite: graphics::Image,
    grass_background: graphics::Image,
    ghetto_timer: i32,
    main_theme: audio::Source,
    battle_theme: audio::Source,
    theme: bool,
    text_font: graphics::Font,
    little_font: graphics::Font,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            pos_x: 0.0,
            pos_y: 380.0,
            player_health: 100,
            player_noomba: 1,
            hitbox_size: 110.0,
            combat_mode: false,
            num1: false,
            player_turn: true,
            sprite: graphics::Image::new(_ctx, "/mainplayer.png").unwrap(),
            speed: 20.0,
            up: false,
            down: false,
            left: false,
            right: false,
            enemy_x: 600.0,
            enemy_y: 380.0,
            enemy_health: 100,
            enemy_noomba: 1,
            enemy_dead: false,
            enemy_sprite: graphics::Image::new(_ctx, "/enemy1.png").unwrap(),
            grass_background: graphics::Image::new(_ctx, "/grass_background.png").unwrap(),
            ghetto_timer: 30,
            main_theme: audio::Source::new(_ctx, "/maintheme.ogg")?,
            battle_theme: audio::Source::new(_ctx, "/battletheme.ogg")?,
            theme: true,
            text_font: graphics::Font::new(_ctx, "/DejaVuSerif.ttf", 35)?,
            little_font: graphics::Font::new(_ctx, "/DejaVuSerif.ttf", 20)?,
        };
        s.main_theme.play().unwrap();
        Ok(s)
    }
    fn player_use_shadow_attack(&mut self, player: bool) -> i32 {
        if self.player_noomba > 0 && player {
            self.player_noomba = self.player_noomba - 1;
            let damage = 50;
            damage
        } else if self.enemy_noomba > 0 && !player{
            self.enemy_noomba = self.enemy_noomba - 1;
            println!("Enemy Noomba is now: {}", self.enemy_noomba,);
            let damage = 50;
            damage
        } else {
            println!("Not Enough Noomba!", );
            0
        }
    }
}

impl event::EventHandler for MainState {
    fn resize_event(&mut self, ctx: &mut Context, _width: u32, _height: u32) {
        let rect = ggez::graphics::Rect::new(0.0, 0.0, 1920.0, 1080.0);
        ggez::graphics::set_screen_coordinates(ctx, rect).unwrap();
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        // println!(
        //     "Key pressed: {:?}, modifier {:?}, repeat: {}",
        //     keycode, keymod, repeat
        // );
        match keycode {
            Keycode::Num1 => {
                self.num1 = true;
            }
            Keycode::Up => {
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
        // println!(
        //     "Key pressed: {:?}, modifier {:?}, repeat: {}",
        //     keycode, keymod, repeat
        // );
        match keycode {
            Keycode::Num1 => {
                self.num1 = false;
            }
            Keycode::Up => {
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
        let hitbox_difference_x = self.pos_x - self.enemy_x;
        let hitbox_difference_y = self.pos_y - self.enemy_y;

        let absolute_value_hitbox_difference_x = hitbox_difference_x.abs();
        let absolute_value_hitbox_difference_y = hitbox_difference_y.abs();

        if absolute_value_hitbox_difference_x < (self.hitbox_size)
            && absolute_value_hitbox_difference_y < (self.hitbox_size)
            && !self.enemy_dead
        {
            // println!("Combat!");
            self.combat_mode = true;
            if self.num1
                && self.player_turn
                && self.ghetto_timer > 30
                && self.player_health > 0
                && self.enemy_health > 0           // Player's turn
            {
                self.enemy_health = self.enemy_health - self.player_use_shadow_attack(true);  // Takes in if the user is player, and outputs the damage, auto calculates noomba cost
                println!("Enemy Health is now: {}", self.enemy_health);
                self.player_noomba = self.player_noomba + 1;
                self.player_turn = false;
                self.ghetto_timer = 0;
            } else if !self.player_turn              // Enemy's turn
                && self.ghetto_timer > 30
                && self.enemy_health > 0
                && self.player_health > 0
            {
                self.player_health = self.player_health - self.player_use_shadow_attack(false);
                println!("Player Health is now: {}", self.player_health);
                self.enemy_noomba = self.enemy_noomba + 1;
                self.player_turn = true;
                self.ghetto_timer = 0;
            } else if self.enemy_health < 1 {
                self.combat_mode = false;
                self.enemy_dead = true;
            } else {
                self.ghetto_timer = self.ghetto_timer + 1;
            }
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
            graphics::set_background_color(ctx, graphics::BLACK);
        }
        let backgroundpoint = graphics::Point2::new(0.0, 0.0);
        let point = graphics::Point2::new(self.pos_x, self.pos_y);
        let enemy_point = graphics::Point2::new(self.enemy_x, self.enemy_y);

        graphics::clear(ctx);
        if !self.combat_mode {
            let text = graphics::Text::new(ctx, "Use Arrow Keys To Move", &self.text_font)?;
            let text_point = graphics::Point2::new(0.0, 0.0);
            graphics::draw(ctx, &self.grass_background, backgroundpoint, 0.0).unwrap();
            graphics::draw(ctx, &text, text_point, 0.0)?;
            graphics::draw(ctx, &self.enemy_sprite, enemy_point, 0.0).unwrap();
            graphics::draw(ctx, &self.sprite, point, 0.0).unwrap();
        } else {
            let Card1 = graphics::Image::new(ctx, "/card1.png").unwrap();
            let Card1_point = graphics::Point2::new(760.0, 540.0);
            let Card1_point_enemy = graphics::Point2::new(760.0, 0.0);

            let player_noomba_str = format!("Player Noomba: {}", self.player_noomba);
            let enemy_noomba_str = format!("Enemy Noomba: {}", self.enemy_noomba);

            let player_health_str = format!("Player Health: {}", self.player_health);
            let enemy_health_str = format!("Enemy Health: {}", self.enemy_health);

            let card_text = graphics::Text::new(ctx, "Deal 50 damage to enemy", &self.little_font)?;
            let card_text_usage = graphics::Text::new(ctx, "Press 1 to use (1 Noomba)", &self.little_font)?;
            let card_text_usage_point = graphics::Point2::new(800.0, 800.0);
            let card_text_point = graphics::Point2::new(800.0, 840.0);

            let enemy_noomba_text = graphics::Text::new(ctx, &enemy_noomba_str, &self.text_font)?;
            let player_noomba_text = graphics::Text::new(ctx, &player_noomba_str, &self.text_font)?;
            let enemy_noomba_text_point = graphics::Point2::new(1400.0, 0.0);
            let player_noomba_text_point = graphics::Point2::new(120.0, 900.0);

            let enemy_health_text = graphics::Text::new(ctx, &enemy_health_str, &self.text_font)?;
            let player_health_text = graphics::Text::new(ctx, &player_health_str, &self.text_font)?;
            let enemy_health_text_point = graphics::Point2::new(1400.0, 200.0);
            let player_health_text_point = graphics::Point2::new(120.0, 700.0);

            let card_text_enemy = graphics::Text::new(ctx, "Enemy card", &self.text_font)?;
            let card_text_enemy_point = graphics::Point2::new(760.0, 0.0);

            let battle_enemy_sprite_point = graphics::Point2::new(1200.0, 0.0);
            let battle_player_sprite_point = graphics::Point2::new(320.0, 900.0);

            graphics::draw(ctx, &self.enemy_sprite, battle_enemy_sprite_point, 0.0).unwrap();
            graphics::draw(ctx, &self.sprite, battle_player_sprite_point, 0.0).unwrap();


            graphics::draw(ctx, &Card1, Card1_point, 0.0).unwrap();
            graphics::draw(ctx, &Card1, Card1_point_enemy, 0.0).unwrap();

            graphics::draw(ctx, &card_text, card_text_point, 0.0)?;
            graphics::draw(ctx, &card_text_usage, card_text_usage_point, 0.0)?;
            graphics::draw(ctx, &card_text_enemy, card_text_enemy_point, 0.0)?;

            graphics::draw(ctx, &enemy_noomba_text, enemy_noomba_text_point, 0.0)?;
            graphics::draw(ctx, &player_noomba_text, player_noomba_text_point, 0.0)?;

            graphics::draw(ctx, &enemy_health_text, enemy_health_text_point, 0.0)?;
                graphics::draw(ctx, &player_health_text, player_health_text_point, 0.0)?;

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
