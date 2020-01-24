use std::time::SystemTime;

use crate::tetromino::{self, Tetromino, TetrominoGenerator};

pub const LEVEL_TIMES: [u32; 10] = [1000, 850, 700, 600, 500, 400, 300, 250, 221, 190];
pub const LEVEL_LINES: [u32; 10] = [20, 40, 60, 80, 100, 120, 140, 160, 180, 200];

pub struct Game {
    pub game_map: Vec<Vec<u8>>,
    pub current_level: u32,
    pub score: u32,
    pub num_lines: u32,
    pub current_piece: Option<Tetromino>,
}

impl Game {
    pub fn new() -> Game {
        let mut game_map = Vec::new();
        for _ in 0..16 {
            game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        Game {
            game_map,
            current_level: 1,
            score: 0,
            num_lines: 0,
            current_piece: None,
        }
    }

    pub fn update_score(&mut self, points: u32) {
        self.score += points;
    }

    pub fn increase_level(&mut self) {
        self.current_level += 1;
    }

    pub fn increase_line(&mut self) {
        self.num_lines += 1;
        if self.num_lines > LEVEL_LINES[self.current_level as usize - 1] {
            self.increase_level();
        }
    }

    pub fn create_new_tetromino(&self) -> Tetromino {
        static mut PREV: u8 = 7;
        let mut rand_n = rand::random::<u8>() % 7;
        if unsafe { PREV } == rand_n {
            rand_n = rand::random::<u8>() % 7;
        }
        unsafe {
            PREV = rand_n;
        }
        match rand_n {
            0 => tetromino::TetrominoI::new(),
            1 => tetromino::TetrominoJ::new(),
            2 => tetromino::TetrominoL::new(),
            3 => tetromino::TetrominoO::new(),
            4 => tetromino::TetrominoS::new(),
            5 => tetromino::TetrominoZ::new(),
            6 => tetromino::TetrominoT::new(),
            _ => unreachable!(),
        }
    }

    pub fn check_lines(&mut self) {
        let mut y = 0;
        let mut points_to_add = 0;

        while y < self.game_map.len() {
            let mut complete = true;

            for x in &self.game_map[y] {
                if *x == 0 {
                    complete = false;
                    break;
                }
            }
            if complete == true {
                points_to_add += self.current_level;
                self.game_map.remove(y);
                y -= 1;
            }
            y += 1;
        }
        if self.game_map.len() == 0 {
            points_to_add += 1000;
        }
        self.update_score(points_to_add);
        while self.game_map.len() < 16 {
            self.increase_line();
            self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
    }

    pub fn make_permanent(&mut self) {
        if let Some(ref mut piece) = self.current_piece {
            let mut shift_y = 0;

            while shift_y < piece.states[piece.current_state as usize].len()
                && piece.y + shift_y < self.game_map.len()
            {
                let mut shift_x = 0;

                while shift_x < piece.states[piece.current_state as usize][shift_y].len()
                    && (piece.x + shift_x as isize)
                        < self.game_map[piece.y + shift_y].len() as isize
                {
                    if piece.states[piece.current_state as usize][shift_y][shift_x] != 0 {
                        let x = piece.x + shift_x as isize;
                        self.game_map[piece.y + shift_y][x as usize] =
                            piece.states[piece.current_state as usize][shift_y][shift_x];
                    }
                    shift_x += 1;
                }
                shift_y += 1;
            }
        }
        self.check_lines();
        self.current_piece = None;
    }
}

pub fn is_time_over(game: &Game, timer: &SystemTime) -> bool {
    match timer.elapsed() {
        Ok(elapsed) => {
            let milliseconds = elapsed.as_secs() as u32 * 1000 + elapsed.subsec_nanos() / 1_000_000;
            milliseconds > LEVEL_TIMES[game.current_level as usize + 1]
        }
        Err(_) => false,
    }
}
