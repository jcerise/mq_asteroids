mod screen;

use std::default;
use crate::screen::*;

use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

extern crate rand;
use rand::{Rng};

trait GameObject {
    fn update(&mut self);
    fn draw(&mut self);
}

struct Ship<'a> {
    texture: &'a Texture2D,
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
    rect: Rect,
}

impl GameObject for Ship<'_> {
    fn update(&mut self) {
        self.position += self.velocity;

        // Wrap the screen
        if self.position.x > screen_width() {
            self.position.x = 0.0;
        }

        if self.position.x < 0.0 {
            self.position.x = screen_width()
        }

        if self.position.y > screen_height() {
            self.position.y = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = screen_height()
        }

        self.velocity *= 0.99;
        self.rect.move_to(self.position);
    }
    fn draw(&mut self) {
        let draw_params = DrawTextureParams{
            rotation: self.rotation,
            ..Default::default()
        };
        draw_texture_ex(self.texture, self.position.x, self.position.y, WHITE, draw_params);
    }
}

struct Bullet<'a> {
    texture: &'a Texture2D,
    position: Vec2,
    velocity: Vec2,
    rect: Rect,
    created_at: f64,
    collided: bool,
}

impl GameObject for Bullet<'_> {
    fn update(&mut self) {
        self.position += self.velocity;
        self.rect.move_to(self.position);
    }

    fn draw(&mut self) {
        draw_texture(self.texture, self.position.x, self.position.y, WHITE);
    }
}

struct Asteroid<'a> {
    texture: &'a Texture2D,
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
    rect: Rect,
    collided: bool,
    large: bool,
}

impl GameObject for Asteroid<'_> {
    fn update(&mut self) {
        self.position += self.velocity;

        //Wrap the screen
        if self.position.x > screen_width() {
            self.position.x = 0.0;
        }

        if self.position.x < 0.0 {
            self.position.x = screen_width()
        }

        if self.position.y > screen_height() {
            self.position.y = 0.0;
        }

        if self.position.y < 0.0 {
            self.position.y = screen_height()
        }

        if self.rotation >= 0.0 {
            self.rotation += 0.01
        } else {
            self.rotation += -0.01
        }

        self.rect.move_to(self.position);
    }

    fn draw(&mut self) {
        let draw_params = DrawTextureParams{
            rotation: self.rotation,
            ..Default::default()
        };
        draw_texture_ex(self.texture, self.position.x, self.position.y, WHITE, draw_params);
    }
}

struct GameStartScreen;

impl GameScreen for GameStartScreen {
    fn display(&mut self) {
        let title = "MQ ASTEROIDS";
        let instructions = "Press <ENTER> to start";
        draw_text_ex(
            title,
            screen_width() / 2. - measure_text(title, None, 50, 1.0).width / 2.0,
            screen_height() / 2.,
            TextParams{
                font_size: 50,
                color: WHITE,
                ..Default::default()
            });
        draw_text_ex(
            instructions,
            screen_width() / 2. - measure_text(instructions, None, 30, 1.0).width / 2.0,
            screen_height() / 2. + 50.,
            TextParams{
                font_size: 30,
                color: WHITE,
                ..Default::default()
            });
    }

    fn handle_controls(&mut self, previous_screen: Option<Box<dyn GameScreen>>) -> Option<Box<dyn GameScreen>> {
        if is_key_pressed(KeyCode::Enter) {
            Some(Box::new(GamePlayScreen));
        }
        None
    }
}

struct GamePlayScreen;

impl GameScreen for GamePlayScreen {
    fn display(&mut self) {
        let text = "This is the gameplay screen";
        let instructions = "Press <ESCAPE> to go back";
        draw_text_ex(
            instructions,
            screen_width() / 2. - measure_text(instructions, None, 50, 1.0).width / 2.0,
            screen_height() / 2.,
            TextParams{
                font_size: 50,
                color: WHITE,
                ..Default::default()
            });

    }

    fn handle_controls(&mut self, previous_screen: Option<Box<dyn GameScreen>>) -> Option<Box<dyn GameScreen>> {
        if is_key_pressed(KeyCode::Escape) {
            Some(previous_screen)
        }
        None
    }
}


fn conf() -> Conf {
    Conf {
        window_title: "MQ Asteroids".to_string(),
        window_width: 640,
        window_height: 480,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let ship_texture: Texture2D = load_texture("resources/ship.png").await.unwrap();
    let bullet_texture: Texture2D = load_texture("resources/bullet.png").await.unwrap();
    let asteroid_texture: Texture2D = load_texture("resources/asteroid_1.png").await.unwrap();
    let large_asteroid_texture_1: Texture2D = load_texture("resources/asteroid_2.png").await.unwrap();
    let large_asteroid_texture_2: Texture2D = load_texture("resources/asteroid_3.png").await.unwrap();
    let large_asteroid_texture_3: Texture2D = load_texture("resources/asteroid_4.png").await.unwrap();


    let mut rng = rand::thread_rng();
    let  mut screen_manager: ScreenManager = ScreenManager{
        current_screen: Box::new(GameStartScreen),
        previous_screen: Some(Box::new(GameStartScreen))

    };

    let ship_position = Vec2::new(screen_width() / 2., screen_height() / 2.);
    let mut ship: Ship = Ship {
        texture: &ship_texture,
        position: ship_position,
        velocity: Vec2::new(0.0, 0.0),
        rotation: 0.0,
        rect: Rect::new(ship_position[0], ship_position[1], ship_texture.width() / 2., ship_texture.height() / 2.),
    };

    let asteroids: &mut Vec<Asteroid> = &mut Vec::new();
    let large_asteroid_textures: &mut Vec<Texture2D> = &mut Vec::new();
    large_asteroid_textures.push(large_asteroid_texture_1);
    large_asteroid_textures.push(large_asteroid_texture_2);
    large_asteroid_textures.push(large_asteroid_texture_3);

    // Create five asteroids, and set them in motion with random velocity
    for _ in 0..8 {
        let rotation = rng.gen_range(-10.0..=10.0);
        let pos = Vec2::new(rng.gen_range(0.0..=screen_width()), rng.gen_range(0.0..=screen_height()));
        let tex = large_asteroid_textures.choose().unwrap();
        asteroids.push( Asteroid{
            texture: tex,
            position: pos,
            velocity: Vec2::from_angle(rotation) * rng.gen_range(0.1..=1.0),
            rotation,
            rect: Rect::new(pos[0], pos[1], tex.width() / 2., tex.height() / 2.),
            collided: false,
            large: true,
        });
    }

    let bullets: &mut Vec<Bullet> = &mut Vec::new();
    let mut last_shot = get_time();

    loop {
        let frame_t = get_time();
        clear_background(BLACK);

        screen_manager.current_screen.display();
        if let Some(new_screen) = screen_manager.current_screen.handle_controls(screen_manager.previous_screen) {
            screen_manager.change_screen(new_screen);
        }

        // if is_key_down(KeyCode::Up) {
        //     let acceleration = Vec2::from_angle(ship.rotation) * 0.1;
        //     ship.velocity += acceleration;
        // }
        //
        // if is_key_down(KeyCode::Right) {
        //     ship.rotation = ship.rotation + 0.1;
        // }
        //
        // if is_key_down(KeyCode::Left) {
        //     ship.rotation = ship.rotation - 0.1;
        // }
        //
        // if is_key_down(KeyCode::Space) && frame_t - last_shot > 0.2 {
        //     let pos = ship.position + Vec2::from_angle(ship.rotation);
        //     bullets.push(Bullet {
        //         texture: &bullet_texture,
        //         position: pos,
        //         velocity: Vec2::from_angle(ship.rotation) * 15.,
        //         rect: Rect::new(pos[0], pos[1], &bullet_texture.width() / 2., &bullet_texture.height() / 2.),
        //         created_at: frame_t,
        //         collided: false
        //     });
        //     last_shot = frame_t;
        // }
        //
        // ship.update();
        // ship.draw();
        //
        // for asteroid in asteroids.iter_mut() {
        //     asteroid.draw();
        //     asteroid.update();
        // }
        //
        // for bullet in &mut bullets.iter_mut() {
        //     bullet.draw();
        //     bullet.update();
        // }
        //
        // // Check bullet and asteroid collision
        // let new_asteroids: &mut Vec<Asteroid> = &mut Vec::new();
        // for asteroid in asteroids.iter_mut() {
        //     for bullet in bullets.iter_mut() {
        //         if asteroid.rect.overlaps(&bullet.rect) {
        //             asteroid.collided = true;
        //             bullet.collided = true;
        //
        //             if asteroid.large {
        //                 // Break the asteroid into several smaller asteroids
        //                 for _ in 2..rng.gen_range(3..=8) {
        //                     let rotation = rng.gen_range(-10.0..=10.0);
        //                     let pos = asteroid.position;
        //                     new_asteroids.push( Asteroid{
        //                         texture: &asteroid_texture,
        //                         position: pos,
        //                         velocity: Vec2::from_angle(rotation) * rng.gen_range(0.1..=1.0),
        //                         rotation,
        //                         rect: Rect::new(pos[0], pos[1], &asteroid_texture.width() / 2., &asteroid_texture.height() / 2.),
        //                         collided: false,
        //                         large: false,
        //                     });
        //                 }
        //             }
        //         }
        //     }
        //
        // }
        //
        // bullets.retain(|bullet| bullet.created_at + 1.5 > frame_t && !bullet.collided);
        // asteroids.retain(|asteroid| !asteroid.collided);
        //
        // asteroids.append(new_asteroids);

        next_frame().await
    }
}
