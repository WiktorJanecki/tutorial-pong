use macroquad::prelude::*;

#[macroquad::main("Tytuł Aplikacji")]
async fn main() {
    let ball_size = 50.;
    let mut ball_pos = Vec2::new(100.,100.);
    let mut ball_vel = Vec2::new(200.,200.);

    let paddle_width = 20.;
    let paddle_height = 200.;
    let paddle_speed = 500.;
    let mut paddle_pos = Vec2::new(0.,0.);

    let mut score = 0;

    loop{
        let delta_time = get_frame_time();

        if is_key_down(KeyCode::Up){
            paddle_pos.y  -= paddle_speed * delta_time;
            
        }
        if is_key_down(KeyCode::Down){

            paddle_pos. y += paddle_speed * delta_time;
        }
        
        ball_pos += ball_vel * delta_time;

        if ball_pos.x > screen_width() - ball_size - paddle_width{
            ball_vel.x = -ball_vel.x;
            score+=1;
            if ball_pos.y < paddle_pos.y - ball_size || ball_pos.y > paddle_pos.y + paddle_height {
                ball_pos = Vec2::new(100., 100.);
                score = 0;
            }
        }
        
        if ball_pos.y > screen_height() - ball_size - paddle_width{
            ball_vel.y = -ball_vel.y;
        }
        
        if ball_pos.x < paddle_width{
            ball_vel.x = -ball_vel.x;
            score+=1;
            if ball_pos.y < paddle_pos.y - ball_size || ball_pos.y > paddle_pos.y + paddle_height {
                ball_pos = Vec2::new(100., 100.);
                score = 0;
            }
        }

        if ball_pos.y < 0.{
            ball_vel.y = -ball_vel.y;
        }


        // before this line show drawing string, error message with just getfps and with tostring
        draw_text(format!("SCORE: {}", score).as_str(), 32., 48., 32., WHITE);
        
        draw_rectangle(ball_pos.x, ball_pos.y, ball_size, ball_size, WHITE);
        draw_rectangle(0.,                            paddle_pos.y, paddle_width, paddle_height, WHITE);
        draw_rectangle(screen_width() - paddle_width, paddle_pos.y, paddle_width, paddle_height, WHITE);

        next_frame().await
    }
}
