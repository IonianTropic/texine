use std::fs;

use sdl2::{render::Canvas, video::Window, event::Event, keyboard::Keycode, EventPump, ttf::{Font, Sdl2TtfContext}, pixels::Color, render::TextureQuery, rect::Rect};


pub struct Game {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    ttf_context: Sdl2TtfContext,
    _prompt: String,
    _command: String,
}

impl Game {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.clear();

        let event_pump = sdl_context.event_pump().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();

        Self {
            canvas,
            event_pump,
            ttf_context, 
            _prompt: String::new(),
            _command: String::new(),
        }
    }

    pub fn run(&mut self) {
        self.text_demo();

        'running: loop {
            // print prompt

            // get user input
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            // handle user input (update)

            // print response

        }
    }

    fn text_demo(&mut self) {        
        // Create surface with desired text
        let font = self.ttf_context
            .load_font("./assets/fonts/OpenSans-Regular.ttf", 12)
            .unwrap();

        let surface = font
            .render("hello, wrold!")
            .blended(Color::RGB(255,255,255))
            .map_err(|e| e.to_string())
            .unwrap();

        // Convert surface into texture
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        // Display texture
        let TextureQuery {width, height, ..} = texture.query();
        let target_rect = Rect::new(
            0,
            0,
            width,
            height
        );

        self.canvas.copy(&texture, None, target_rect).unwrap();
        self.canvas.present();
    }
}
