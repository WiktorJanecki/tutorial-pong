use macroquad::prelude::*;

#[macroquad::main("Tytuł Aplikacji")]
async fn main() {
    let ball_size = 50.;
    let mut ball_pos = Vec2::new(0.,0.);
    let mut ball_vel = Vec2::new(200.,200.);

    loop{
        let delta_time = get_frame_time();
        ball_pos += ball_vel * delta_time;

        if ball_pos.x > screen_width() - ball_size{
            ball_vel.x = -ball_vel.x;
        }
        
        if ball_pos.y > screen_height() - ball_size{
            ball_vel.y = -ball_vel.y;
        }
        
        if ball_pos.x < 0.{
            ball_vel.x = -ball_vel.x;
        }

        if ball_pos.y < 0.{
            ball_vel.y = -ball_vel.y;
        }


        // before this line show drawing string, error message with just getfps and with tostring
        draw_text(get_fps().to_string().as_str(), 16., 32., 32., WHITE);
        
        draw_rectangle(ball_pos.x, ball_pos.y, ball_size, ball_size, WHITE);

        next_frame().await
    }
}
