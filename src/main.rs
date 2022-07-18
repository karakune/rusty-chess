mod chess_entities;
mod chess_components;
mod chess_systems;

// extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::time::Duration;
use crate::chess_components::CPosition;
use crate::chess_entities::{GameState, initialize_game_state};

use crate::chess_systems::get_piece;

const WINDOW_SIZE: u32 = 600;
const SQUARE_SIZE: u32 = WINDOW_SIZE / 8;
const SPRITE_SIZE: u32 = 60;
const LIGHT_SQUARE: Color = Color::RGB(249, 228, 183);
const DARK_SQUARE: Color = Color::RGB(179, 147, 119);

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rusty Chess", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let _image_context = image::init(InitFlag::PNG)?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("resources/ChessPieces.png")?;


    let game_state = initialize_game_state();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                | Event::MouseButtonDown {
                    ..
                } => {
                    let x = 0;
                    let y = 0;
                    get_piece(x, y);
                },
                _ => {}
            }
        }

        canvas.clear();
        render_board(&mut canvas);
        render_pieces(&mut canvas, &texture, &game_state)?;
        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}

fn render_board(canvas: &mut WindowCanvas) {
    for i in 0..8 {
        for j in 0..8 {
            if (i + j) % 2 == 0 {
                canvas.set_draw_color(LIGHT_SQUARE);
            }
            else {
                canvas.set_draw_color(DARK_SQUARE);
            }
            canvas.fill_rect(Rect::new((SQUARE_SIZE * j) as i32, (SQUARE_SIZE * i) as i32, SQUARE_SIZE, SQUARE_SIZE)).unwrap();
        }
    }
    canvas.set_draw_color(Color::BLACK);
}

fn render_pieces(canvas: &mut WindowCanvas, texture: &Texture, game_state: &GameState) -> Result<(), String>{
    for piece in &game_state.e_pieces {
        let position = game_state.c_positions.get(*piece).unwrap().unwrap();
        canvas.copy(texture, game_state.c_sprites.get(*piece).unwrap().unwrap().sprite, Rect::new(position.x * SQUARE_SIZE as i32, position.y * SQUARE_SIZE as i32, SQUARE_SIZE, SQUARE_SIZE))?;
    }

    Ok(())
}