use macroquad::prelude::*;

#[macroquad::main("Tytuł Aplikacji")]
async fn main() {
    let mut pos_x = 200.;
    let mut pos_y = 200.;

    let vel_x = 50.;
    let vel_y = 50.;
    loop{
        let delta_time = get_frame_time();
        pos_x += delta_time * vel_x;
        pos_y += delta_time * vel_y;
        


        // before this line show drawing string, error message with just getfps and with tostring
        draw_text(get_fps().to_string().as_str(), 16., 32., 32., WHITE);
        
        draw_rectangle(pos_x, pos_y, 50., 50., WHITE);

        next_frame().await
    }
}
