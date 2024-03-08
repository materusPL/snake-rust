use rand::prelude::*;
use raylib::prelude::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

const RECT_SIZE: i32 = 20;

const BOARD_HEIGHT: i32 = HEIGHT / RECT_SIZE;
const BOARD_WIDTH: i32 = WIDTH / RECT_SIZE;
#[derive(Copy, Clone)]
enum RectType {
    HEAD,
    BODY,
    EMPTY,
    APPLE,
}
#[derive(Copy, Clone)]
enum Direction {
    TOP,
    DOWN,
    LEFT,
    RIGHT,
}
#[derive(Copy, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}
fn gen_apple(game_data: &GameData) -> Position {
    let mut pos: Position = Position { x: 0, y: 0 };
    let mut done = false;
    while !done {
        done = true;
        pos = Position {
            x: thread_rng().gen_range(0..BOARD_WIDTH),
            y: thread_rng().gen_range(0..BOARD_HEIGHT),
        };
        if game_data.head == pos {
            done = false
        } else {
            for tail_seg in &game_data.tail {
                if (tail_seg.x == pos.x) && (tail_seg.y == pos.y) {
                    done = false;
                    break;
                }
            }
        }
    }
    pos
}
struct GameData {
    board: [RectType; (BOARD_HEIGHT * BOARD_HEIGHT) as usize],
    head: Position,
    tail: Vec<Position>,
    direction: Direction,
    last_direction: Direction,
    game_over: bool,
    score: u32,
    apple: Position,
}
fn update_game(game_data: &mut GameData) {
    game_data.last_direction = game_data.direction;
    let old_head_pos = game_data.head;
    match game_data.direction {
        Direction::DOWN => game_data.head.y += 1,
        Direction::TOP => game_data.head.y -= 1,
        Direction::LEFT => game_data.head.x -= 1,
        Direction::RIGHT => game_data.head.x += 1,
    }
    if game_data.head.x >= BOARD_WIDTH {
        game_data.head.x = 0
    } else if game_data.head.x < 0 {
        game_data.head.x = BOARD_WIDTH - 1
    } else if game_data.head.y < 0 {
        game_data.head.y = BOARD_HEIGHT - 1
    } else if game_data.head.y >= BOARD_HEIGHT {
        game_data.head.y = 0
    }
    if game_data.head == game_data.apple {
        game_data.apple = gen_apple(game_data);
        game_data.score += 1;
        game_data.tail.push(game_data.tail.last().copied().unwrap());
    }

    let last_tail = game_data.tail.last().copied().unwrap();
    for tail_id in (1..game_data.tail.len()).rev() {
        game_data.tail[tail_id] = game_data.tail[tail_id - 1];
        if game_data.head == game_data.tail[tail_id] {
            game_data.game_over = true;
        }
        game_data.board
            [(game_data.tail[tail_id].x + game_data.tail[tail_id].y * BOARD_HEIGHT) as usize] =
            RectType::BODY;
    }
    game_data.board[(last_tail.x + last_tail.y * BOARD_HEIGHT) as usize] = RectType::EMPTY;
    game_data.tail[0] = old_head_pos;
    game_data.board[(old_head_pos.x + old_head_pos.y * BOARD_HEIGHT) as usize] = RectType::BODY;
    game_data.board[(game_data.head.x + game_data.head.y * BOARD_HEIGHT) as usize] = RectType::HEAD;
    game_data.board[(game_data.apple.x + game_data.apple.y * BOARD_HEIGHT) as usize] =
        RectType::APPLE;
}
fn create_game_data() -> GameData {
    let mut game_data = GameData {
        board: [RectType::EMPTY; (BOARD_HEIGHT * BOARD_WIDTH) as usize],
        head: Position {
            x: BOARD_WIDTH / 2,
            y: BOARD_HEIGHT / 2,
        },
        tail: vec![
            Position {
                x: BOARD_WIDTH / 2,
                y: BOARD_HEIGHT / 2,
            };
            3
        ],
        direction: Direction::RIGHT,
        last_direction: Direction::LEFT,
        game_over: false,
        score: 0,
        apple: Position { x: 0, y: 0 },
    };
    game_data.apple = gen_apple(&game_data);
    game_data
}
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Snake")
        .vsync()
        .build();

    let show_fps: bool = false;

    let mut time: f64 = 0.0;
    let mut game_data = create_game_data();

    while !rl.window_should_close() {
        let fps = rl.get_fps();
        let frame_time = rl.get_frame_time();
        time += f64::from(frame_time);

        if (rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W))
            && !matches!(game_data.last_direction, Direction::DOWN)
        {
            game_data.direction = Direction::TOP
        }

        if (rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S))
            && !matches!(game_data.last_direction, Direction::TOP)
        {
            game_data.direction = Direction::DOWN
        }
        if (rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A))
            && !matches!(game_data.last_direction, Direction::RIGHT)
        {
            game_data.direction = Direction::LEFT
        }
        if (rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D))
            && !matches!(game_data.last_direction, Direction::LEFT)
        {
            game_data.direction = Direction::RIGHT
        }

        if time >= 0.1 {
            time = 0.0;
            if !game_data.game_over {
                update_game(&mut game_data);
            };
        }
        if rl.is_key_down(KeyboardKey::KEY_ENTER) && game_data.game_over {
            game_data = create_game_data();
            time = 0.0;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGRAY);
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let color: Option<Color>;
                match game_data.board[(x + y * BOARD_WIDTH) as usize] {
                    RectType::HEAD => color = Option::Some(Color::PURPLE),
                    RectType::BODY => color = Option::Some(Color::DARKPURPLE),
                    RectType::APPLE => color = Option::Some(Color::GREEN),
                    RectType::EMPTY => color = None,
                };
                match color {
                    Some(c) => {
                        d.draw_rectangle(x * RECT_SIZE, y * RECT_SIZE, RECT_SIZE, RECT_SIZE, c)
                    }
                    None => {}
                }
            }
        }
        if !game_data.game_over {
            d.draw_text(
                format!("Score: {}", game_data.score).as_str(),
                20,
                10,
                20,
                Color::RED,
            );
        }

        if game_data.game_over {
            d.draw_text("Game Over", WIDTH / 2 - 100, HEIGHT / 2, 40, Color::WHITE);
            d.draw_text(
                format!("Score: {}", game_data.score).as_str(),
                WIDTH / 2 - 50,
                HEIGHT / 2 + 50,
                20,
                Color::WHITE,
            );
            d.draw_text(
                "Press Enter",
                WIDTH / 2 - 40,
                HEIGHT / 2 + 75,
                10,
                Color::WHITE,
            );
        }
        if show_fps {
            d.draw_text(fps.to_string().as_str(), WIDTH - 50, 10, 20, Color::BLUE)
        }
    }
}
