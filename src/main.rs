use std::collections::VecDeque;

use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

const TILE_SIZE: f32 = 16.0;
const MAP_SIZE: f32 = 20.0;

const SCREEN_WIDTH: f32 = TILE_SIZE * MAP_SIZE;
const SCREEN_HEIGHT: f32 = TILE_SIZE * MAP_SIZE;

const GAME_SPEED: u32 = 5;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Food {
    position: Vector2,
}
impl Food {
    fn new() -> Self {
        Self {
            position: Vector2::new(
                get_random_value::<i32>(0, (MAP_SIZE as i32) - 1) as f32,
                get_random_value::<i32>(0, (MAP_SIZE as i32) - 1) as f32,
            ),
        }
    }
    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            self.position.x as i32 * TILE_SIZE as i32,
            self.position.y as i32 * TILE_SIZE as i32,
            TILE_SIZE as i32,
            TILE_SIZE as i32,
            Color::RED,
        );
    }
    fn respawn(&mut self) {
        self.position = Vector2::new(
            get_random_value::<i32>(0, (MAP_SIZE as i32) - 1) as f32,
            get_random_value::<i32>(0, (MAP_SIZE as i32) - 1) as f32,
        );
    }
}

struct Snake {
    body: VecDeque<Vector2>,
    direction: Direction,
    next_direction: Direction,
}

impl Snake {
    fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_back(Vector2::new(4.0, MAP_SIZE * 0.5));
        body.push_back(Vector2::new(3.0, MAP_SIZE * 0.5));
        body.push_back(Vector2::new(2.0, MAP_SIZE * 0.5));

        Self {
            body,
            direction: Direction::Right,
            next_direction: Direction::Right,
        }
    }
    fn update(&mut self) {
        let _ = self.body.pop_back();
        let mut temp_head = self.body[0].clone();

        if self.next_direction != self.direction {
            // prevent 180 degree movement
            match self.direction {
                Direction::Down => {
                    if self.next_direction != Direction::Up {
                        self.direction = self.next_direction;
                    }
                }
                Direction::Up => {
                    if self.next_direction != Direction::Down {
                        self.direction = self.next_direction;
                    }
                }
                Direction::Left => {
                    if self.next_direction != Direction::Right {
                        self.direction = self.next_direction;
                    }
                }
                Direction::Right => {
                    if self.next_direction != Direction::Left {
                        self.direction = self.next_direction;
                    }
                }
            }
        }

        // Wrap the snake around the map
        match self.direction {
            Direction::Down => {
                temp_head.y += 1.0;
                if temp_head.y >= MAP_SIZE {
                    temp_head.y = 0.0;
                }
                self.body.push_front(temp_head);
            }
            Direction::Up => {
                temp_head.y -= 1.0;
                if temp_head.y < 0.0 {
                    temp_head.y = MAP_SIZE - 1.0;
                }
                self.body.push_front(temp_head);
            }
            Direction::Left => {
                temp_head.x -= 1.0;
                if temp_head.x < 0.0 {
                    temp_head.x = MAP_SIZE - 1.0;
                }
                self.body.push_front(temp_head);
            }
            Direction::Right => {
                temp_head.x += 1.0;
                if temp_head.x >= MAP_SIZE {
                    temp_head.x = 0.0;
                }
                self.body.push_front(temp_head);
            }
        }
    }
    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let mut color = Color::GREEN;
        for tile in self.body.iter() {
            d.draw_rectangle(
                tile.x as i32 * TILE_SIZE as i32,
                tile.y as i32 * TILE_SIZE as i32,
                TILE_SIZE as i32,
                TILE_SIZE as i32,
                color,
            );
            color = Color::RAYWHITE;
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Snake")
        .build();

    rl.set_target_fps(30);

    let mut frame_count = 0;
    let mut snake = Snake::new();
    let mut food = Food::new();

    while !rl.window_should_close() {
        if rl.is_key_pressed(KEY_UP) {
            snake.next_direction = Direction::Up;
        }
        if rl.is_key_pressed(KEY_DOWN) {
            snake.next_direction = Direction::Down;
        }
        if rl.is_key_pressed(KEY_LEFT) {
            snake.next_direction = Direction::Left;
        }
        if rl.is_key_pressed(KEY_RIGHT) {
            snake.next_direction = Direction::Right;
        }

        if frame_count & GAME_SPEED == 0 {
            snake.update();
        }
        for tile in snake.body.iter().skip(1) {
            if snake.body[0] == *tile {
                return;
            }
        }
        if snake.body[0] == food.position {
            food.respawn();
            let tmp = snake.body.back().unwrap().clone();
            snake.body.push_back(tmp);
        }

        let mut d = rl.begin_drawing(&thread);
        snake.draw(&mut d);
        food.draw(&mut d);
        frame_count += 1;

        d.clear_background(Color::GRAY);
    }
}
