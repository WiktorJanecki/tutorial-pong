use macroquad::{audio::{load_sound, play_sound_once}, prelude::*};

#[macroquad::main("Tytuł Aplikacji")]
async fn main() {
    // TODO: ADD GAMEOVER SOUND AND POINT SOUND
    let paddle_txt= load_texture("assets/paddle.png").await.unwrap();
    let ball_txt= load_texture("assets/ball.png").await.unwrap();
    let font = load_ttf_font("assets/SQUARED2.ttf").await.unwrap();
    let ball_sound = load_sound("assets/ping.wav").await.unwrap();
    let gameover_sound = load_sound("assets/gameover.wav").await.unwrap();

    let ball_size = 30.;
    let mut ball_pos = Vec2::new(100.,100.);
    let mut ball_vel = Vec2::new(200.,200.);

    let paddle_width = 40.;
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
            if ball_pos.y < paddle_pos.y - ball_size || ball_pos.y > paddle_pos.y + paddle_height {
                ball_pos = Vec2::new(100., 100.);
                score = 0;
                play_sound_once(&gameover_sound);
            }
            else{
                ball_vel.x = -ball_vel.x;
                play_sound_once(&ball_sound);
                score+=1;
            }
        }
        
        if ball_pos.y > screen_height() - ball_size{
            ball_vel.y = -ball_vel.y;
        }
        
        if ball_pos.x < paddle_width{
            if ball_pos.y < paddle_pos.y - ball_size || ball_pos.y > paddle_pos.y + paddle_height {
                ball_pos = Vec2::new(100., 100.);
                score = 0;
                play_sound_once(&gameover_sound);
            }
            else{
                ball_vel.x = -ball_vel.x;
                score+=1;
                play_sound_once(&ball_sound);
            }
        }

        if ball_pos.y < 0.{
            ball_vel.y = -ball_vel.y;
        }

        clear_background(LIGHTGRAY);
        let center = get_text_center(format!("{}", score).as_str(), Some(&font), 150, 1., 0.);
        draw_text_ex(format!("{}", score).as_str(), screen_width()/2. - center.x, screen_height()/2. - center.y,TextParams { 
            font: Some(&font), font_size: 150, color: WHITE, ..Default::default() });
        
        draw_texture(&ball_txt, ball_pos.x, ball_pos.y,WHITE);
        
        draw_texture(&paddle_txt, 4.,paddle_pos.y,WHITE);
        draw_texture(&paddle_txt, screen_width() - paddle_width - 4.0 ,paddle_pos.y,WHITE);

        next_frame().await
    }
}
