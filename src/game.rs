use std::fs;

use sdl2::{render::Canvas, video::Window, event::Event, keyboard::Keycode, EventPump, ttf::Font, pixels::Color};


pub struct Game {
    canvas: Canvas<Window>,
    event_pump: EventPump,
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
        canvas.present();
    
        let event_pump = sdl_context.event_pump().unwrap();

        let sdl_ttf_context = sdl2::ttf::init().unwrap();

        let font = sdl_ttf_context
            .load_font("./assets/fonts/OpenSans-Regular.ttf", 12)
            .unwrap();
        let partial_rendering = font.render("hello, wrold!");
        partial_rendering.solid(Color::RGB(255, 255, 255)).unwrap();
        Self {
            canvas,
            event_pump,
            _prompt: String::new(),
            _command: String::new(),
        }
    }

    pub fn run(&mut self) {
        
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
            self.canvas.clear();
            self.canvas.present();
        }
    }

    fn _print_prompt(prompt: String) {

    }
}
