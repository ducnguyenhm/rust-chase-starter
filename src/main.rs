use rand::Rng;
use raylib::prelude::*;
use std::time::Instant;

fn main() {
    // Variables for the screen size.
    let width = 640;
    let height = 480;
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Hello, World")
        .build();

    // Player data
    let mut player_position = Vector2::new(25.0, 25.0);
    let size: f32 = 10.0;
    let speed: f32 = 300.0;

    // goal data
    let mut goal_position = random_vector2(width, height);
    while player_position.distance_to(goal_position) < 15.0 {
        goal_position = random_vector2(width, height);
    }
    let goal_size: f32 = 30.0;

    let mut last_time = Instant::now();
    let success  = 5;
    let mut score = 0;

    while !rl.window_should_close() {
        // update delta time.
        let temp = Instant::now();
        let delta = (temp - last_time).as_secs_f32();
        last_time = temp;

        let mut direction = Vector2::new(0.0, 0.0);

        // Get user input
        if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN) {
            direction = direction + Vector2 { x: 0.0, y: 1.0 };
        }
        if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP) {
            direction = direction + Vector2 { x: 0.0, y: -1.0 };
        }
        if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT) {
            direction = direction + Vector2 { x: -1.0, y: 0.0 };
        }
        if rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            direction = direction + Vector2 { x: 1.0, y: 0.0 };
        }

        // use the unit vector for direction
        direction.normalize();
        player_position = player_position + direction * (speed * delta);

        // Keep player within window bounds
        if player_position.x < size {
            player_position.x = size;
        }
        if player_position.x > width as f32 - size {
            player_position.x = width as f32 - size;
        }
        if player_position.y < size {
            player_position.y = size;
        }
        if player_position.y > height as f32 - size {
            player_position.y = height as f32 - size;
        }

        // check for collision
        if player_position.distance_to(goal_position) < 15.0 {
            println!("hit!!");
            score = score + 1;
            goal_position = random_vector2(width, height);
            if score == success {
                break;
            } 

        }

        // Peform drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text(&format!("Score: {}", score), 12, height - 20, 20, Color::BLACK);

        d.draw_circle_v(player_position, size, Color::BLUE);

        let goal_offset_x = (goal_position.x - (goal_size / 2.0)) as i32;
        let goal_offset_y = (goal_position.y - (goal_size / 2.0)) as i32;
        d.draw_rectangle(
            goal_offset_x,
            goal_offset_y,
            goal_size as i32,
            goal_size as i32,
            Color::GOLD,
        );
    }

    // Win screen
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("YOU WIN!", width / 2 - 100, height / 2 - 40, 40, Color::BLACK);
        d.draw_text(&format!("Final Score: {}/{}", score, success), width / 2 - 120, height / 2 + 20, 30, Color::BLACK);
        d.draw_text("Press ESC to exit", width / 2 - 100, height / 2 + 60, 20, Color::BLACK);
    }
}

fn random_vector2(width: i32, height: i32) -> Vector2 {
    let mut rng = rand::rng();
    let x = rng.random_range(0..width);
    let y = rng.random_range(0..height);

    Vector2 {
        x: x as f32,
        y: y as f32,
    }
}
