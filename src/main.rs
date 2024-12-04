use macroquad::prelude::*;

#[macroquad::main("Tytuł Aplikacji")]
async fn main() {
    let ball_size = 50.;

    let mut pos_x = 200.;
    let mut pos_y = 200.;

    let mut vel_x = 200.;
    let mut vel_y = 200.;
    loop{
        let delta_time = get_frame_time();
        pos_x += delta_time * vel_x;
        pos_y += delta_time * vel_y;

        if pos_x > screen_width() - ball_size{
            vel_x = -vel_x;
        }
        
        if pos_y > screen_height() - ball_size{
            vel_y = -vel_y;
        }
        
        if pos_x < 0.{
            vel_x = -vel_x;
        }

        if pos_y < 0.{
            vel_y = -vel_y;
        }


        // before this line show drawing string, error message with just getfps and with tostring
        draw_text(get_fps().to_string().as_str(), 16., 32., 32., WHITE);
        
        draw_rectangle(pos_x, pos_y, ball_size, ball_size, WHITE);

        next_frame().await
    }
}
