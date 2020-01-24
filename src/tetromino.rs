pub type Piece = Vec<Vec<u8>>;
pub type States = Vec<Piece>;

pub trait TetrominoGenerator {
    fn new() -> Tetromino;
}

pub struct Tetromino {
    pub states: States,
    pub x: isize,
    pub y: usize,
    pub current_state: u8,
}

pub struct TetrominoI;

impl TetrominoGenerator for TetrominoI {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![
                vec![
                    vec![1, 1, 1, 1],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetrominoJ;

impl TetrominoGenerator for TetrominoJ {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![
                vec![
                    vec![2, 2, 2, 0],
                    vec![2, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 2, 0, 0],
                    vec![0, 2, 0, 0],
                    vec![0, 2, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 2, 0],
                    vec![2, 2, 2, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![2, 0, 0, 0],
                    vec![2, 0, 0, 0],
                    vec![2, 2, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetrominoL;

impl TetrominoGenerator for TetrominoL {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![
                vec![
                    vec![3, 3, 3, 0],
                    vec![0, 0, 3, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 3, 0, 0],
                    vec![0, 3, 0, 0],
                    vec![3, 3, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 0, 0, 0],
                    vec![3, 3, 3, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![3, 3, 0, 0],
                    vec![3, 0, 0, 0],
                    vec![3, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetrominoO;

impl TetrominoGenerator for TetrominoO {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![vec![
                vec![4, 4, 0, 0],
                vec![4, 4, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]],
            x: 5,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetrominoS;

impl TetrominoGenerator for TetrominoS {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![
                vec![
                    vec![0, 5, 5, 0],
                    vec![5, 5, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 5, 0, 0],
                    vec![0, 5, 5, 0],
                    vec![0, 0, 5, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetrominoZ;

impl TetrominoGenerator for TetrominoZ {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![
                vec![
                    vec![6, 6, 0, 0],
                    vec![0, 6, 6, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 6, 0],
                    vec![0, 6, 6, 0],
                    vec![0, 6, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

pub struct TetrominoT;

impl TetrominoGenerator for TetrominoT {
    fn new() -> Tetromino {
        Tetromino {
            states: vec![
                vec![
                    vec![7, 7, 7, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 0, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![7, 7, 7, 0],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                vec![
                    vec![0, 7, 0, 0],
                    vec![0, 7, 7, 0],
                    vec![0, 7, 0, 0],
                    vec![0, 0, 0, 0],
                ],
            ],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}

impl Tetromino {
    pub fn rotate(&mut self, game_map: &[Vec<u8>]) {
        let mut tmp_state = self.current_state + 1;
        if tmp_state as usize >= self.states.len() {
            tmp_state = 0;
        }
        let x_pos = [0, -1, 1, -2, 2, -3];
        for x in x_pos.iter() {
            if self.test_position(game_map, tmp_state as usize, self.x, self.y) == true {
                self.current_state = tmp_state;
                self.x += *x;
                break;
            }
        }
    }

    pub fn change_position(&mut self, game_map: &[Vec<u8>], new_x: isize, new_y: usize) -> bool {
        if self.test_position(game_map, self.current_state as usize, new_x, new_y) == true {
            self.x = new_x as isize;
            self.y = new_y;
            true
        } else {
            false
        }
    }

    pub fn test_current_position(&self, game_map: &[Vec<u8>]) -> bool {
        self.test_position(game_map, self.current_state as usize, self.x, self.y)
    }

    pub fn test_position(
        &self,
        game_map: &[Vec<u8>],
        tmp_state: usize,
        x: isize,
        y: usize,
    ) -> bool {
        for decal_y in 0..4 {
            for decal_x in 0..4 {
                let x = x + decal_x;
                if self.states[tmp_state][decal_y][decal_x as usize] != 0
                    && (y + decal_y >= game_map.len()
                        || x < 0
                        || x as usize >= game_map[y + decal_y].len()
                        || game_map[y + decal_y][x as usize] != 0)
                {
                    return false;
                }
            }
        }
        true
    }
}
