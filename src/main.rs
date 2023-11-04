use macroquad::prelude::*;

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

    let ship_rotation: f32 = 0.1;

    loop {
        clear_background(BLACK);

        let draw_params = DrawTextureParams{
            rotation: ship_rotation,
            ..Default::default()
        };

        // The divider is static and does not move, its purpose is to visually divide the screen
        draw_texture_ex(&ship_texture, screen_width() / 2., screen_height() / 2., WHITE, draw_params);
        next_frame().await
    }
}

fn handle_input(p: &mut MoveableObject) {
    let mut delta = 0.;
    if is_key_down(KeyCode::Right) {
        delta += -1.;
    }
    if is_key_down(KeyCode::Left) {
        delta += 1.;
    }

    if (p.point.y + delta) <= 0. || (p.point.y + delta) >= (screen_height() - p.texture.height()) {
        delta = 0.
    }

    p.update_position(p.point.x, p.point.y + delta)
}
