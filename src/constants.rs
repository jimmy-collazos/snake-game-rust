use piston_window::types::Color;

// window
pub const WINDOW_BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
pub const WINDOW_TITLE: &str = "Snake Game";

// canvas
pub const CANVAS_SIZE: (i32, i32) = (30, 30);
pub const CANVAS_BLOCK_SIZE: f64 = 30.0;

pub const CANVAS_FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
pub const CANVAS_BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
pub const CANVAS_GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

// game
pub const GAME_MOVING_PERIOD: f64 = 0.1;
pub const GAME_RESTART_TIME: f64 = 1.0;

// snake
pub const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];