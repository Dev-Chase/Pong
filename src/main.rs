use macroquad::prelude::*;

// Importing Modules
mod player;
mod ball;
use player::Player;
use ball::Ball;

enum GameState {
    Start,
    Gameover,
    Play,
    Pause,
    Setup
}

fn play(player_a: &mut Player, player_b: &mut Player, ball: &mut Ball)-> GameState{
    // Updating Players and Drawing their Rects
    player_a.update();
    player_b.update();
    player_a.draw();
    player_b.draw();

    // Checking to see If Space key Pressed  and if so pause the game
    if is_key_released(KeyCode::Space) {
        return GameState::Pause;
    }

    // Checking to See if a Player Won
    if player_a.score == 10f32 || player_b.score == 10f32 {
        return GameState::Gameover;
    }

    // Updating Ball and Drawing it's Rect
    ball.update(player_a, player_b);
    ball.draw();

    // Measuring text Sections
    let player_a_score = measure_text(format!("{}", player_a.score).as_str(), Default::default(), 30u16, 1f32);
    let player_b_score = measure_text(format!("{}", player_b.score).as_str(), Default::default(), 30u16, 1f32);

    // Drawing Text Sections
    draw_text(format!("{}", player_a.score).as_str(), screen_width()*0.5f32-player_a_score.width-15f32, 20f32, 30f32, WHITE);
    draw_text(format!("{}", player_b.score).as_str(), screen_width()*0.5f32+player_b_score.width, 20f32, 30f32, WHITE);

    // Drawing Border Line
    draw_line(screen_width()*0.5f32-1f32, 0f32, screen_width()*0.5f32-1f32, screen_height(), 4f32, WHITE);
    GameState::Play
}

fn pause(player_a: &Player, player_b: &Player) -> GameState {
    // Getting Text Section Dimensions
    let pause_text_dimensions = measure_text("Press Space or Click to Unpause", Default::default(), 25u16, 1f32);
    let player_a_score = measure_text(format!("{}", player_a.score).as_str(), Default::default(), 30u16, 1f32);
    let player_b_score = measure_text(format!("{}", player_b.score).as_str(), Default::default(), 30u16, 1f32);

    // Drawing Text Sections
    draw_text(format!("{}", player_a.score).as_str(), screen_width()*0.5f32-player_a_score.width-15f32, 20f32, 30f32, WHITE);
    draw_text(format!("{}", player_b.score).as_str(), screen_width()*0.5f32+player_b_score.width, 20f32, 30f32, WHITE);
    draw_text("Press Space or Click to Unpause", screen_width()*0.5f32-pause_text_dimensions.width*0.5f32, screen_height()*0.5f32-pause_text_dimensions.height*0.5f32, 30f32, WHITE);

    // Drawing Border Line
    draw_line(screen_width()*0.5f32-1f32, 0f32, screen_width()*0.5f32-1f32, screen_height(), 4f32, WHITE);

    // Checking to see If Space key Pressed or Left mouse button clicked and if so pause the game
    if is_key_released(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
        return GameState::Play;
    }

    GameState::Pause
}

fn setup(player_a: &mut Player, player_b: &mut Player, ball: &mut Ball)-> GameState {
    *player_a = Player::new(5f32, screen_height()*0.5f32-11f32, 11f32, 55f32, WHITE, KeyCode::W, KeyCode::S, '1');
    *player_b = Player::new(screen_width()-5f32-9f32, screen_height()*0.5f32-11f32, 11f32, 55f32, WHITE, KeyCode::Up, KeyCode::Down, '2');
    *ball = Ball::new(screen_width()*0.5f32, screen_height()*0.5f32, WHITE);
    GameState::Play
}

fn gameover(player_a: &Player, player_b: &Player) -> GameState {
    // Creating Winner Variable and Measuring Winner Text
    let winner_text = format!("Player {} Won!", match player_a.score as i32 {
        10 => player_a.id,
        _ => player_b.id
    });

    // Drawing Border Line
    draw_line(screen_width()*0.5f32-1f32, 0f32, screen_width()*0.5f32-1f32, screen_height(), 4f32, WHITE);

    // Getting Text Section Dimensions
    let restart_text_dimensions = measure_text("Press Space or Click to Restart", Default::default(), 30u16, 1f32);
    let player_a_score = measure_text(format!("{}", player_a.score).as_str(), Default::default(), 30u16, 1f32);
    let player_b_score = measure_text(format!("{}", player_b.score).as_str(), Default::default(), 30u16, 1f32);
    let winner_text_dimensions = measure_text(winner_text.as_str(), Default::default(), 30u16, 1f32);
    
    // Drawing Text Sections
    draw_text(format!("{}", player_a.score).as_str(), screen_width()*0.5f32-player_a_score.width-15f32, 20f32, 30f32, WHITE);
    draw_text(format!("{}", player_b.score).as_str(), screen_width()*0.5f32+player_b_score.width, 20f32, 30f32, WHITE);
    draw_text("Press Space or Click to Restart", screen_width()*0.5f32-restart_text_dimensions.width*0.5f32, screen_height()*0.5f32+restart_text_dimensions.height+winner_text_dimensions.height*0.5f32, 30f32, WHITE);
    draw_text(winner_text.as_str(), screen_width()*0.5f32-winner_text_dimensions.width*0.5f32, screen_height()*0.5f32-winner_text_dimensions.height*0.5f32, 30f32, WHITE);



    // Checking to See if Space key pressed or Left mouse button clicked
    if is_key_released(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
        return GameState::Setup
    }
    GameState::Gameover
}

fn start() -> GameState {
    // Getting Text Section Dimensions
    let start_text_dimensions = measure_text("Press Space or Click to Start", Default::default(), 25u16, 1f32);

    // Drawing Text Sections
    draw_text("Press Space or Click to Start", screen_width()*0.5f32-start_text_dimensions.width*0.5f32, screen_height()*0.5f32-start_text_dimensions.height*0.5f32, 25f32, WHITE);

    // Checking to see if Space Key Pressed or Left mouse button clicked
    if is_key_released(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
        return GameState::Setup
    }
    GameState::Start
}

// Creating a Function to Configurate the Window
fn create_window_conf() -> Conf {
    Conf {
        window_title: String::from("Pong"),
        fullscreen: false,
        window_width: 500,
        window_height: 350,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(create_window_conf())]
async fn main() {
    // Creating Player and Ball Variables
    let mut player_a = Player::new(5f32, screen_height()*0.5f32-11f32, 11f32, 55f32, WHITE, KeyCode::W, KeyCode::S, '1');
    let mut player_b = Player::new(screen_width()-5f32-9f32, screen_height()*0.5f32-11f32, 11f32, 55f32, WHITE, KeyCode::Up, KeyCode::Down, '2');
    let mut ball = Ball::new(screen_width()*0.5f32, screen_height()*0.5f32, WHITE);

    // Creating GameState Variable
    let mut game_state = GameState::Start;

    let mut minimum_frame_time: f32;
    let mut frame_time: f32;
    let mut time_to_sleep = 0f32;

    loop {
        clear_background(BLACK);

        game_state = match game_state {
            GameState::Start => start(),
            GameState::Pause => pause(&player_a, &player_b),
            GameState::Play => play(&mut player_a, &mut player_b, &mut ball),
            GameState::Gameover => gameover(&player_a, &player_b),
            GameState::Setup => setup(&mut player_a, &mut player_b, &mut ball),
        };

        // Capping the Frame Rate
        minimum_frame_time = 1. / 180.; // 60 FPS
        frame_time = get_frame_time()-time_to_sleep*0.001f32;
        time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
        std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));

        next_frame().await
    }
}