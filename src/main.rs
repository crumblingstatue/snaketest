use macroquad::prelude::*;
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const CELL_SIZE: usize = 64;

fn draw_cell(x: usize, y: usize, color: Color) {
    draw_rectangle(
        (x * CELL_SIZE) as f32,
        (y * CELL_SIZE) as f32,
        CELL_SIZE as f32,
        CELL_SIZE as f32,
        color,
    )
}

struct Snake {
    head_x: usize,
    head_y: usize,
    dir: Dir,
    segments: Vec<(usize, usize)>,
}

impl Snake {
    fn new() -> Self {
        Self {
            head_x: rand::gen_range(0, WIDTH),
            head_y: rand::gen_range(0, HEIGHT),
            dir: Dir::Right,
            segments: Vec::default(),
        }
    }
    fn move_(&mut self) {
        let mut new_x = self.head_x;
        let mut new_y = self.head_y;
        match self.dir {
            Dir::Left => {
                if self.head_x != 0 {
                    new_x -= 1;
                }
            }
            Dir::Right => new_x += 1,
            Dir::Up => {
                if self.head_y != 0 {
                    new_y -= 1;
                }
            }
            Dir::Down => new_y += 1,
        }
        self.head_x = new_x;
        self.head_y = new_y;
    }
}

#[macroquad::main("snake")]
async fn main() {
    let mut snake = Snake::new();
    let mut frames_elapsed = 0;
    let mut apple = (0, 0);
    let mut prev_head_x = 0;
    let mut prev_head_y = 0;
    loop {
        if is_key_pressed(KeyCode::Left) {
            snake.dir = Dir::Left;
        }
        if is_key_pressed(KeyCode::Right) {
            snake.dir = Dir::Right;
        }
        if is_key_pressed(KeyCode::Up) {
            snake.dir = Dir::Up;
        }
        if is_key_pressed(KeyCode::Down) {
            snake.dir = Dir::Down;
        }
        draw_snake(&snake);
        draw_cell(apple.0, apple.1, RED);
        if snake.head_x == apple.0 && snake.head_y == apple.1 {
            apple.0 = rand::gen_range(0, WIDTH);
            apple.1 = rand::gen_range(0, HEIGHT);
            snake.segments.push((snake.head_x, snake.head_y));
        }
        if frames_elapsed % 10 == 0 {
            snake.move_();
            for i in (0..snake.segments.len()).rev() {
                if i == 0 {
                    snake.segments[i].0 = prev_head_x;
                    snake.segments[i].1 = prev_head_y;
                } else {
                    snake.segments[i].0 = snake.segments[i - 1].0;
                    snake.segments[i].1 = snake.segments[i - 1].1;
                }
            }
            prev_head_x = snake.head_x;
            prev_head_y = snake.head_y;
        }
        next_frame().await;
        frames_elapsed += 1;
    }
}

fn draw_snake(snake: &Snake) {
    draw_cell(snake.head_x, snake.head_y, GREEN);
    for &(x, y) in &snake.segments {
        draw_cell(x, y, GREEN);
    }
}
