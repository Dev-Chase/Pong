use macroquad::prelude::*;

const SPEED: f32 = 6f32;

pub struct Player {
    pub rect: Rect,
    colour: Color,
    up_ctrl: KeyCode,
    down_ctrl: KeyCode,
    pub score: f32,
    pub id: char,
}

impl Player {
    pub fn new(
        x_coord: f32,
        y_coord: f32,
        width: f32,
        height: f32,
        color: Color,
        up: KeyCode,
        down: KeyCode,
        char: char,
    ) -> Player {
        Player {
            rect: Rect {
                x: x_coord,
                y: y_coord,
                w: width,
                h: height,
            },
            colour: color,
            up_ctrl: up,
            down_ctrl: down,
            score: 0f32,
            id: char,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.colour,
        );
    }

    pub fn update(&mut self) {
        let y_dir = is_key_down(self.down_ctrl) as i8 - is_key_down(self.up_ctrl) as i8;

        if (self.rect.y + (y_dir as f32 * SPEED) > 1f32)
            && (self.rect.y + self.rect.h + (y_dir as f32 * SPEED) < screen_height() - 1f32)
        {
            self.rect
                .move_to(vec2(self.rect.x, self.rect.y + (y_dir as f32 * SPEED)));
        }
    }
}
