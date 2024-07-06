use raylib::{color::Color, drawing::RaylibDraw};

fn main() {
    // Initialize the Raylib window
    let (mut rl, thread) = raylib::init().size(800, 600).title("Hello World").build();

    while !rl.window_should_close() {
        // Begin drawing on the window
        let mut d = rl.begin_drawing(&thread);

        // Clear the background with a white color
        d.clear_background(Color::WHITE);

        // Draw the "Hello World" text at position (12, 12) with a font size of 20 and black color
        d.draw_text("Hello World", 12, 12, 20, Color::BLACK);
    }
}
