use crate::player::Player;
use macroquad::prelude::*;

extern crate rand;
use rand::Rng;

const VELS: [[f32; 2]; 4] = [[8f32, 2f32], [7f32, 3f32], [6f32, 4f32], [5f32, 5f32]];
const MAYBE_NEGATIVE: [f32; 2] = [1f32, -1f32];

pub struct Ball {
    circle: Circle,
    vel: [f32; 2],
    colour: Color,
    x_neg: f32,
    y_neg: f32,
}

impl Ball {
    pub fn new(x_coord: f32, y_coord: f32, clr: Color) -> Ball {
        let rand_vels = VELS[rand::thread_rng().gen_range(0f32..VELS.len() as f32) as usize];
        Ball {
            circle: Circle::new(x_coord, y_coord, 7f32),
            vel: [rand_vels[0] - 1.5f32, rand_vels[1] - 1.5f32],
            colour: clr,
            x_neg: MAYBE_NEGATIVE[rand::thread_rng().gen_range(0f32..2f32) as usize],
            y_neg: MAYBE_NEGATIVE[rand::thread_rng().gen_range(0f32..2f32) as usize],
        }
    }

    pub fn draw(&self) {
        draw_circle(self.circle.x, self.circle.y, self.circle.r, self.colour);
    }

    fn reset(&mut self, side: usize) {
        self.circle
            .move_to(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32));
        let rand_vels = VELS[rand::thread_rng().gen_range(0f32..VELS.len() as f32) as usize];
        self.vel = [rand_vels[0] - 1.5f32, rand_vels[1] - 1.5f32];
        self.x_neg = MAYBE_NEGATIVE[side];
        self.y_neg = MAYBE_NEGATIVE[rand::thread_rng().gen_range(0f32..=1f32) as usize];
    }

    pub fn update(&mut self, player_a: &mut Player, player_b: &mut Player) {
        if self.will_hit_paddle(player_a) {
            self.circle.x = player_a.rect.right() + self.circle.r;
            self.x_neg *= -1f32;
            self.vel = VELS[rand::thread_rng().gen_range(0f32..VELS.len() as f32) as usize];
        } else if self.will_hit_paddle(player_b) {
            self.circle.x = player_b.rect.left() - self.circle.r;
            self.x_neg *= -1f32;
            self.vel = VELS[rand::thread_rng().gen_range(0f32..VELS.len() as f32) as usize];
        }

        if self.circle.y + self.circle.r + self.vel[1] <= 0f32 {
            self.y_neg *= -1f32;
        }
        if self.circle.y + self.circle.r + self.vel[1] >= screen_height() {
            self.y_neg *= -1f32;
        }

        let hit_side = self.will_score();
        if hit_side.1 {
            if hit_side.0 == 0f32 {
                player_b.score += 1f32;
            } else if hit_side.0 == 1f32 {
                player_a.score += 1f32;
            }
            self.reset(hit_side.0 as usize);
        }

        self.circle.x += self.vel[0] * self.x_neg;
        self.circle.y += self.vel[1] * self.y_neg;
    }

    fn will_hit_paddle(&self, paddle: &mut Player) -> bool {
        if paddle.rect.contains(self.circle.point()) {
            return true;
        }
        false
    }

    fn will_score(&self) -> (f32, bool) {
        if self.circle.x - self.circle.r + self.vel[0] <= 0f32 {
            return (0f32, true);
        } else if self.circle.x + self.circle.r + self.vel[0] >= screen_width() {
            return (1f32, true);
        }
        (0f32, false)
    }
}
