use macroquad::prelude::*;

trait GameObject {
    fn update(&mut self);
    fn draw(&mut self);
}

#[derive(Clone, Copy)]
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
}

impl GameObject for Bullet<'_> {
    fn update(&mut self) {
        self.position += self.velocity;
    }

    fn draw(&mut self) {
        draw_texture(self.texture, self.position.x, self.position.y, WHITE);
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
    let mut ship: Ship = Ship {
        texture: &ship_texture,
        position: ship_position,
        velocity: Vec2::new(0.0, 0.0),
        rotation: 0.0,
        rect: Rect::new(ship_position[0], ship_position[1], ship_texture.width() / 2., ship_texture.height() / 2.),
    };

    let bullets: &mut Vec<Bullet> = &mut Vec::new();
    let mut last_shot = get_time();

    loop {
        let frame_t = get_time();
        clear_background(BLACK);

        if is_key_down(KeyCode::Up) {
            let acceleration = Vec2::from_angle(ship.rotation) * 0.1;
            ship.velocity += acceleration;
        }

        if is_key_down(KeyCode::Right) {
            ship.rotation = ship.rotation + 0.1;
        }

        if is_key_down(KeyCode::Left) {
            ship.rotation = ship.rotation - 0.1;
        }

        if is_key_down(KeyCode::Space) && frame_t - last_shot > 0.2 {
            let rot_vec = Vec2::new(ship.rotation.sin(), -ship.rotation.cos());
            bullets.push(Bullet {
                texture: &bullet_texture,
                position: (ship.position + Vec2::from_angle(ship.rotation)),
                velocity: Vec2::from_angle(ship.rotation) * 20.,
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
