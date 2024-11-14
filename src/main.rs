use macroquad::prelude::*;
use std::time::Instant;


#[macroquad::main("MyGame")]
async fn main() {
    const FORCE_OF_GRAVITY: f32 = 2.0;

    let mut x_pos = (screen_width() / 2.0) - 60.0;
    let mut y_pos = 100.0;
    loop {
        // Handle inputs
        if is_key_down(KeyCode::Right) {
            x_pos += 5.0;
        }

        if is_key_down(KeyCode::Left) {
            x_pos -= 5.0;
        }

        if is_key_down(KeyCode::Down) {
            y_pos += 5.0;
        }

        if is_key_down(KeyCode::Space) {
            y_pos -= 15.0;
        }

        y_pos += FORCE_OF_GRAVITY;

        let start = Instant::now();
        clear_background(BLACK);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(x_pos, y_pos, 120.0, 60.0, WHITE);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        println!("{:?}", start.elapsed());
        next_frame().await
    }
}
