use macroquad::prelude::*;

#[macroquad::main("Tytuł Aplikacji")]
async fn main() {
    let pos_x = 200.;
    let pos_y = 200.;
    loop{
        // before this line show drawing string, error message with just getfps and with tostring
        draw_text(get_fps().to_string().as_str(), 16., 32., 32., WHITE);
        
        draw_rectangle(pos_x, pos_y, 50., 50., WHITE);

        next_frame().await
    }
}
