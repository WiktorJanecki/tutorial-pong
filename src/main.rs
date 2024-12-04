use macroquad::prelude::*;

#[macroquad::main("Tytuł Aplikacji")]
async fn main() {
    let ball_size = 50.;
    let mut ball_pos = Vec2::new(100.,100.);
    let mut ball_vel = Vec2::new(200.,200.);

    let paddle_width = 20.;
    let paddle_height = 200.;
    let paddle_speed = 500.;
    let mut first_paddle_pos = Vec2::new(0.,0.);
    let mut second_paddle_pos = Vec2::new(screen_width()-paddle_width,0.);

    loop{
        let delta_time = get_frame_time();

        if is_key_down(KeyCode::Up){
            first_paddle_pos.y  -= paddle_speed * delta_time;
            second_paddle_pos.y -= paddle_speed * delta_time;
            
        }
        if is_key_down(KeyCode::Down){

            first_paddle_pos. y += paddle_speed * delta_time;
            second_paddle_pos.y += paddle_speed * delta_time;
        }
        
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
        draw_rectangle(first_paddle_pos.x, first_paddle_pos.y, paddle_width, paddle_height, WHITE);
        draw_rectangle(second_paddle_pos.x, second_paddle_pos.y, paddle_width, paddle_height, WHITE);

        next_frame().await
    }
}
