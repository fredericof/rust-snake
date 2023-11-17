extern crate sdl2;

use game_context::GameContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;
use sdl2::video::Window;

mod game_context;

const GRID_X_SIZE: u32 = 40;
const GRID_Y_SIZE: u32 = 30;
const DOT_SIZE_IN_PXS: u32 = 20;

pub struct Renderer {
    canvas: WindowCanvas
}

impl Renderer {
    pub fn new(window: Window ) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_dot(&mut self, point: &game_context::Point) -> Result<(), String> {
        let game_context::Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * DOT_SIZE_IN_PXS as i32,
            y * DOT_SIZE_IN_PXS as i32,
            DOT_SIZE_IN_PXS,
            DOT_SIZE_IN_PXS,
        ))?;
    
        Ok(())
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.draw_background(context);
        self.draw_player(context)?;
        self.draw_food(context)?;
        self.canvas.present();

        Ok(())
    }

    pub fn draw_background(&mut self, context: &GameContext) {
        let color = match context.state {
            game_context::GameState::Playing => Color::RGB(0, 0, 0),
            game_context::GameState::Paused => Color::RGB(30, 30, 30),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_player(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::GREEN);
        for point in &context.player_position {
            self.draw_dot(point)?;
        }
    
        Ok(())
    }

    fn draw_food(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&context.food)?;
        Ok(())
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Snake Game",GRID_X_SIZE * DOT_SIZE_IN_PXS, GRID_Y_SIZE * DOT_SIZE_IN_PXS)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window)?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut context = game_context::GameContext::new();
    
    let mut frame_counter = 0;
    let mut last_key_pressed= Keycode::S;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::W => {
                            if last_key_pressed != Keycode::S && last_key_pressed != Keycode::W {
                                context.move_up();
                            }

                            last_key_pressed = keycode;
                        },
                        Keycode::A => {
                            if last_key_pressed != Keycode::D && last_key_pressed != Keycode::A {
                                context.move_left();
                            }
                            last_key_pressed = keycode;
                        },
                        Keycode::S => {
                            if last_key_pressed != Keycode::W && last_key_pressed != Keycode::S {
                                context.move_down();
                            }
                            last_key_pressed = keycode;
                        },
                        Keycode::D => {
                            if last_key_pressed != Keycode::A && last_key_pressed != Keycode::D {
                                context.move_right();
                            }
                            last_key_pressed = keycode;
                        },
                        Keycode::Escape => {
                            context.toggle_pause();
                            last_key_pressed = keycode;
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32/30 ));

        frame_counter += 1;
        if frame_counter % 3 == 0 {
            context.next_tick();
            frame_counter = 0;
        }

        renderer.draw(&context).unwrap();
    }

    Ok(())
}