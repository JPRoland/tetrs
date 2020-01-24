use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::SystemTime;

use crate::game::Game;

pub fn handle_events(
    game: &mut Game,
    quit: &mut bool,
    timer: &mut SystemTime,
    event_pump: &mut sdl2::EventPump,
) -> bool {
    let mut make_permanent = false;
    if let Some(ref mut piece) = game.current_piece {
        let mut tmp_x = piece.x;
        let mut tmp_y = piece.y;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    *quit = true;
                    break;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    *timer = SystemTime::now();
                    tmp_y += 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    tmp_x += 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    tmp_x -= 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    piece.rotate(&game.game_map);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    let x = piece.x;
                    let mut y = piece.y;

                    while piece.change_position(&game.game_map, x, y + 1) == true {
                        y += 1;
                    }
                    make_permanent = true;
                }
                _ => {}
            }
        }
        if !make_permanent {
            if piece.change_position(&game.game_map, tmp_x, tmp_y) == false && tmp_y != piece.y {
                make_permanent = true;
            }
        }
    }
    if make_permanent {
        game.make_permanent();
        *timer = SystemTime::now();
    }
    make_permanent
}
