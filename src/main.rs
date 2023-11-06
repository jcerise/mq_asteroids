use std::os::unix::raw::time_t;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct MoveableObject<'a> {
    texture: &'a Texture2D,
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
    rect: Rect,
    created_at: f64,
}

impl MoveableObject<'_> {
    fn update_position(&mut self, position: Vec2) {
        self.position += position;
        //self.rect.move_to(Vec2::new(x, y));
    }
    fn update_rotation(&mut self, rot: f32) {self.rotation = rot}
    fn update_velocity(&mut self, acceleration: Vec2) { self.velocity += acceleration }
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
    }
    fn draw(&mut self) {
        let draw_params = DrawTextureParams{
            rotation: self.rotation,
            ..Default::default()
        };
        draw_texture_ex(self.texture, self.position.x, self.position.y, WHITE, draw_params);
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

    let ship_position = Vec2::new(screen_width() / 2., screen_height() / 2.);
    let mut ship: MoveableObject = MoveableObject {
        texture: &ship_texture,
        position: ship_position,
        velocity: Vec2::new(0.0, 0.0),
        rotation: 0.0,
        rect: Rect::new(ship_position[0], ship_position[1], ship_texture.width() / 2., ship_texture.height() / 2.),
        created_at: get_time()
    };

    let bullets: &mut Vec<MoveableObject> = &mut Vec::new();
    let mut last_shot = get_time();

    loop {
        let frame_t = get_time();
        clear_background(BLACK);

        handle_input(&mut ship);

        if is_key_down(KeyCode::Space) && frame_t - last_shot > 0.2 {
            let rot_vec = Vec2::new(ship.rotation.sin(), -ship.rotation.cos());
            bullets.push(MoveableObject {
                texture: &bullet_texture,
                position: (ship.position + Vec2::from_angle(ship.rotation)),
                velocity: Vec2::from_angle(ship.rotation) * 5.,
                rotation: 0.0,
                rect: Default::default(),
                created_at: frame_t,
            });
            last_shot = frame_t;
        }

        ship.update();
        ship.draw();

        for bullet in &mut bullets.iter_mut() {
            bullet.draw();
            bullet.update();
        }

        bullets.retain(|bullet| bullet.created_at + 1.5 > frame_t);

        next_frame().await
    }
}

fn handle_input(p: &mut MoveableObject) {
    let mut rot_delta: f32 = 0.;
    if is_key_down(KeyCode::Right) {
        rot_delta += 0.1;
    }
    if is_key_down(KeyCode::Left) {
        rot_delta += -0.1;
    }


    if is_key_down(KeyCode::Up) {
        let acceleration = Vec2::from_angle(p.rotation) * 0.1;
        p.update_velocity(acceleration);
    }

    p.update_rotation(p.rotation + rot_delta)
}
