extern crate rand;
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use std::fs::File;
use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

mod events;
mod game;
mod tetromino;

const GAME_HEIGHT: usize = 40;
const HIGHSCORE_FILE: &'static str = "scores.txt";
const NUM_HIGHSCORES: usize = 5;

fn main() {
    let mut tetrs = game::Game::new();
    let mut timer = SystemTime::now();
    let sdl_ctx = sdl2::init().expect("Failed to initialize SDL");
    let ttf_ctx = sdl2::ttf::init().expect("SDL TTF init failed");
    let video_subsystem = sdl_ctx
        .video()
        .expect("Failed to initialize video subsystem");

    let width = 1200;
    let height = 800;

    let font = ttf_ctx
        .load_font("assets/ponde___.ttf", 128)
        .expect("Failed to load font");

    let mut event_pump = sdl_ctx.event_pump().expect("Failed to get SDL event pump");

    let grid_x = (width - GAME_HEIGHT as u32 * 10) as i32 / 2;
    let grid_y = (height - GAME_HEIGHT as u32 * 16) as i32 / 2;

    let window = video_subsystem
        .window("Tetrs", width, height)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Failed to create canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let grid = create_texture_rect(
        &mut canvas,
        &texture_creator,
        0,
        0,
        0,
        GAME_HEIGHT as u32 * 10,
        GAME_HEIGHT as u32 * 16,
    )
    .expect("Failed to create grid");

    let border = create_texture_rect(
        &mut canvas,
        &texture_creator,
        255,
        255,
        255,
        GAME_HEIGHT as u32 * 10 + 20,
        GAME_HEIGHT as u32 * 16 + 20,
    )
    .expect("Failed to create border");

    macro_rules! texture {
        ($r:expr, $g:expr, $b:expr) => {
            create_texture_rect(
                &mut canvas,
                &texture_creator,
                $r,
                $g,
                $b,
                GAME_HEIGHT as u32,
                GAME_HEIGHT as u32,
            )
            .unwrap()
        };
    }

    let textures = [
        texture!(255, 69, 69),
        texture!(255, 220, 69),
        texture!(237, 150, 37),
        texture!(171, 99, 237),
        texture!(77, 149, 239),
        texture!(39, 218, 225),
        texture!(45, 216, 47),
    ];

    loop {
        if game::is_time_over(&tetrs, &timer) {
            let mut make_permanent = false;
            if let Some(ref mut piece) = tetrs.current_piece {
                let x = piece.x;
                let y = piece.y + 1;
                make_permanent = !piece.change_position(&tetrs.game_map, x, y);
            }
            if make_permanent {
                tetrs.make_permanent();
            }
            timer = SystemTime::now();
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas
            .copy(
                &border,
                None,
                Rect::new(
                    (width - GAME_HEIGHT as u32 * 10) as i32 / 2 - 10,
                    (height - GAME_HEIGHT as u32 * 16) as i32 / 2 - 10,
                    GAME_HEIGHT as u32 * 10 + 20,
                    GAME_HEIGHT as u32 * 16 + 20,
                ),
            )
            .expect("Couldn't copy texture into window");
        canvas
            .copy(
                &grid,
                None,
                Rect::new(
                    (width - GAME_HEIGHT as u32 * 10) as i32 / 2,
                    (height - GAME_HEIGHT as u32 * 16) as i32 / 2,
                    GAME_HEIGHT as u32 * 10,
                    GAME_HEIGHT as u32 * 16,
                ),
            )
            .expect("Couldn't copy texture into window");

        if tetrs.current_piece.is_none() {
            let current_piece = tetrs.create_new_tetromino();
            if !current_piece.test_current_position(&tetrs.game_map) {
                print_game_info(&tetrs);
                break;
            }
            tetrs.current_piece = Some(current_piece);
        }
        let mut quit = false;

        if !events::handle_events(&mut tetrs, &mut quit, &mut timer, &mut event_pump) {
            if let Some(ref mut piece) = tetrs.current_piece {
                for (line_num, line) in piece.states[piece.current_state as usize]
                    .iter()
                    .enumerate()
                {
                    for (case_num, case) in line.iter().enumerate() {
                        if *case == 0 {
                            continue;
                        }
                        canvas
                            .copy(
                                &textures[*case as usize - 1],
                                None,
                                Rect::new(
                                    grid_x
                                        + (piece.x + case_num as isize) as i32 * GAME_HEIGHT as i32,
                                    grid_y + (piece.y + line_num) as i32 * GAME_HEIGHT as i32,
                                    GAME_HEIGHT as u32,
                                    GAME_HEIGHT as u32,
                                ),
                            )
                            .expect("Couldn't copy texture to canvas");
                    }
                }
            }
        }

        if quit {
            print_game_info(&tetrs);
            break;
        }

        for (line_num, line) in tetrs.game_map.iter().enumerate() {
            for (case_num, case) in line.iter().enumerate() {
                if *case == 0 {
                    continue;
                }
                canvas
                    .copy(
                        &textures[*case as usize - 1],
                        None,
                        Rect::new(
                            grid_x + case_num as i32 * GAME_HEIGHT as i32,
                            grid_y + line_num as i32 * GAME_HEIGHT as i32,
                            GAME_HEIGHT as u32,
                            GAME_HEIGHT as u32,
                        ),
                    )
                    .expect("Couldn't copy texture to canvas");
            }
        }

        display_game_info(
            &tetrs,
            &mut canvas,
            &texture_creator,
            &font,
            width as i32 - grid_x + 15,
        );

        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn create_texture_from_text<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    text: &str,
    r: u8,
    g: u8,
    b: u8,
) -> Option<Texture<'a>> {
    if let Ok(surface) = font.render(text).blended(Color::RGB(r, g, b)) {
        texture_creator.create_texture_from_surface(&surface).ok()
    } else {
        None
    }
}

fn get_rect_from_text(text: &str, x: i32, y: i32) -> Option<Rect> {
    Some(Rect::new(x, y, text.len() as u32 * 20, 30))
}

fn display_game_info<'a>(
    tetrs: &game::Game,
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    start_point: i32,
) {
    let score_text = format!("SCORE: {}", tetrs.score);
    let lines_text = format!("LINES: {}", tetrs.num_lines);
    let level_text = format!("LEVEL: {}", tetrs.current_level);

    let score = create_texture_from_text(&texture_creator, &font, &score_text, 255, 255, 255)
        .expect("Cannot render text");
    let lines = create_texture_from_text(&texture_creator, &font, &lines_text, 255, 255, 255)
        .expect("Cannot render text");
    let level = create_texture_from_text(&texture_creator, &font, &level_text, 255, 255, 255)
        .expect("Cannot render text");

    canvas
        .copy(
            &score,
            None,
            get_rect_from_text(&score_text, start_point, 90),
        )
        .expect("couldn't opy text");
    canvas
        .copy(
            &lines,
            None,
            get_rect_from_text(&score_text, start_point, 125),
        )
        .expect("couldn't opy text");
    canvas
        .copy(
            &level,
            None,
            get_rect_from_text(&score_text, start_point, 160),
        )
        .expect("couldn't opy text");
}

pub fn print_game_info(game: &game::Game) {
    let mut new_highest_highscore = true;
    let mut new_highest_lines_sent = true;
    if let Some((mut highscores, mut lines_sent)) = load_highscores_and_lines() {
        new_highest_highscore = update_vec(&mut highscores, game.score);
        new_highest_lines_sent = update_vec(&mut lines_sent, game.num_lines);
        if new_highest_highscore || new_highest_lines_sent {
            save_highscore_and_lines(&highscores, &lines_sent);
        }
    } else {
        save_highscore_and_lines(&[game.score], &[game.num_lines]);
    }
    println!("Game over...");
    println!(
        "Score:           {}{}",
        game.score,
        if new_highest_highscore {
            " [NEW HIGHSCORE]"
        } else {
            ""
        }
    );
    println!(
        "Number of lines: {}{}",
        game.num_lines,
        if new_highest_lines_sent {
            " [NEW HIGHSCORE]"
        } else {
            ""
        }
    );
    println!("Current level:   {}", game.current_level);
}

fn update_vec(v: &mut Vec<u32>, value: u32) -> bool {
    if v.len() < NUM_HIGHSCORES {
        v.push(value);
        true
    } else {
        for entry in v.iter_mut() {
            if value > *entry {
                *entry = value;
                return true;
            }
        }
        false
    }
}

fn slice_to_string(slice: &[u32]) -> String {
    slice
        .iter()
        .map(|highscore| highscore.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn line_to_slice(line: &str) -> Vec<u32> {
    line.split(" ")
        .filter_map(|nb| nb.parse::<u32>().ok())
        .collect()
}

fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file(HIGHSCORE_FILE) {
        let mut lines = content
            .splitn(2, "\n")
            .map(|line| line_to_slice(line))
            .collect::<Vec<_>>();
        if lines.len() == 2 {
            let (number_lines, highscores) = (lines.pop().unwrap(), lines.pop().unwrap());
            Some((highscores, number_lines))
        } else {
            None
        }
    } else {
        None
    }
}

fn save_highscore_and_lines(highscores: &[u32], lines: &[u32]) -> bool {
    let s_highscores = slice_to_string(highscores);
    let s_lines = slice_to_string(lines);
    write_into_file(&format!("{}\n{}\n", s_highscores, s_lines), HIGHSCORE_FILE).is_ok()
}

fn write_into_file(content: &str, filename: &str) -> io::Result<()> {
    let mut f = File::create(filename)?;
    f.write_all(content.as_bytes())
}

fn read_from_file(filename: &str) -> io::Result<String> {
    let mut f = File::open(filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    r: u8,
    g: u8,
    b: u8,
    width: u32,
    height: u32,
) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, width, height) {
        canvas
            .with_texture_canvas(&mut square_texture, |texture| {
                texture.set_draw_color(Color::RGB(r, g, b));
                texture.clear();
            })
            .expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}
