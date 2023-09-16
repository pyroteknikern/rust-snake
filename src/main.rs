use ncurses::{initscr, raw, curs_set, nodelay, noecho, wgetch, wrefresh, endwin, box_, newwin, wborder, delwin, mvwaddch, mvwaddstr, WINDOW, chtype, CURSOR_VISIBILITY};
use rand::Rng;
use std::thread;
use std::time;

static HEIGHT: u8 = 16;
static WIDTH: u8 = 16;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum GameState {
    Main,
    Pause,
    GameOver,
    Restart,
    Exit,
}

struct Game {
    snake_score: u16,
    snake_direction: Direction,
    snake_body: Vec<[i8; 2]>,
    fruit_pos: [i8; 2],
    game_state: GameState,
}

impl Game {
    fn build() -> Game {
        let mut vec: Vec<[i8; 2]> = Vec::new();
        vec.push([7, 7]);
        Game {
            snake_score: 1,
            snake_direction: Direction::Right,
            snake_body: vec,
            fruit_pos: [1, 1],
            game_state: GameState::Main,
        }
    }

    fn snake_draw(&self, window: WINDOW) {
        for i in 0..self.snake_body.len() {
            mvwaddch(
                window,
                self.snake_body[i][1] as i32,
                self.snake_body[i][0] as i32 * 2,
                'o' as chtype,
            );
        }
    }

    fn fruit_draw(&self, window: WINDOW) {
        mvwaddch(
            window,
            self.fruit_pos[1] as i32,
            self.fruit_pos[0] as i32 * 2,
            'x' as chtype,
        );
    }

    fn snake_move(&mut self) {
        match self.snake_direction {
            Direction::Up => self.snake_body.push([
                self.snake_body[self.snake_body.len() - 1][0],
                self.snake_body[self.snake_body.len() - 1][1] - 1,
            ]),
            Direction::Down => self.snake_body.push([
                self.snake_body[self.snake_body.len() - 1][0],
                self.snake_body[self.snake_body.len() - 1][1] + 1,
            ]),
            Direction::Left => self.snake_body.push([
                self.snake_body[self.snake_body.len() - 1][0] - 1,
                self.snake_body[self.snake_body.len() - 1][1],
            ]),
            Direction::Right => self.snake_body.push([
                self.snake_body[self.snake_body.len() - 1][0] + 1,
                self.snake_body[self.snake_body.len() - 1][1],
            ]),
        }
    }

    fn snake_clear(&self, window: WINDOW) {
        for i in 0..self.snake_body.len() {
            mvwaddch(
                window,
                self.snake_body[i][1] as i32,
                self.snake_body[i][0] as i32 * 2,
                ' ' as chtype,
            );
        }
    }

    fn check_collision(&mut self) {
        for i in 0..self.snake_body.len() - 1 {
            if self.snake_body[self.snake_body.len() - 1] == self.snake_body[i] {
                self.game_state = GameState::GameOver;
            }
        }
        if self.snake_body[self.snake_body.len() - 1][0] <= 0
            || self.snake_body[self.snake_body.len() - 1][0] > WIDTH as i8 + 1
            || self.snake_body[self.snake_body.len() - 1][1] <= 0
            || self.snake_body[self.snake_body.len() - 1][1] > HEIGHT as i8 + 1
        {
            self.game_state = GameState::GameOver;
        }
    }

    fn fruit_spawn(&mut self) {
        self.fruit_pos[0] = rand::thread_rng().gen_range(1..WIDTH - 1) as i8;
        self.fruit_pos[1] = rand::thread_rng().gen_range(1..HEIGHT - 1) as i8;
        for i in 0..self.snake_body.len() {
            if self.fruit_pos == self.snake_body[i] {
                self.fruit_spawn();
            }
        }
        self.snake_score += 1;
    }

    fn fruit_eat(&mut self) {
        for i in 0..self.snake_body.len() {
            if self.fruit_pos == self.snake_body[i] {
                self.fruit_spawn();
                return;
            }
        }
        self.snake_body.remove(0);
    }

    fn handle_key_events(&mut self, ch: i32) {
        match ch {
            LOWER_A => self.snake_direction = Direction::Left,
            LOWER_D => self.snake_direction = Direction::Right,
            LOWER_W => self.snake_direction = Direction::Up,
            LOWER_S => self.snake_direction = Direction::Down,
            LOWER_R => self.game_state = GameState::Restart,
            _ => {}
        }
    }

    fn display_score(&self, window: WINDOW) {
        mvwaddstr(window, 1, 2, "Score: ");
        mvwaddstr(window, 1, 9, "   ");
        mvwaddstr(window, 1, 9, &self.snake_score.to_string());
    }

    fn dead_screen(&self, window: WINDOW) {
        mvwaddstr(window, HEIGHT as i32 / 2, WIDTH as i32, "DU DOG");
        mvwaddstr(window, HEIGHT as i32 / 2 + 1, WIDTH as i32 - 6, "Press (r) to restart");
    }
}

const LOWER_R: i32 = 'r' as i32;
const LOWER_Q: i32 = 'q' as i32;
const LOWER_W: i32 = 'w' as i32;
const LOWER_S: i32 = 's' as i32;
const LOWER_D: i32 = 'd' as i32;
const LOWER_A: i32 = 'a' as i32;
const LOWER_C: i32 = 'c' as i32;

fn main() {
    let mut game = Game::build();

    initscr();
    raw();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    let mut window = create_win(1, 1, HEIGHT as i32 + 2, (WIDTH as i32 + 2) * 2);
    let under_window = create_win(HEIGHT as i32 + 3, 1, 3, HEIGHT as i32 + 2); 
    nodelay(window, true);
    noecho();
    let mut ch = wgetch(window);
    while ch != LOWER_Q {
        ch = wgetch(window);
        game.handle_key_events(ch);
        match game.game_state {
            GameState::Main => {
                game.snake_clear(window);
                game.snake_move();
                game.fruit_eat();
                game.snake_draw(window);
                game.fruit_draw(window);
                game.check_collision();
                game.display_score(under_window);
                if ch == 'p' as i32 {
                    game.game_state = GameState::Pause;
                }
            }
            GameState::GameOver => {
                game.dead_screen(window);
            }
            GameState::Restart => {
                destroy_win(window);
                window = create_win(1, 1, HEIGHT as i32 + 2, (WIDTH as i32 + 2) * 2);
                nodelay(window, true);
                game = Game::build();
            }
            GameState::Pause => {
                if ch == LOWER_C {
                    game.game_state = GameState::Main;
                }
            }
            _ => {}
        }
        wrefresh(under_window);
        wrefresh(window);
        thread::sleep(time::Duration::from_millis(100));
    }
    destroy_win(window);
    endwin();
}

fn create_win(start_y: i32, start_x: i32, h: i32, w: i32) -> WINDOW {
    let win = newwin(h, w, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win
}

fn destroy_win(win: WINDOW) {
    let ch = ' ' as chtype;
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(win);
    delwin(win);
}
