use std::fs;
use crate::display::Display;



use sdl2::{render::Canvas, video::Window, event::Event, keyboard::Keycode, EventPump, ttf::{Font, Sdl2TtfContext}, pixels::Color, render::TextureQuery, rect::Rect};


pub struct Game {
    display: Display,
    _prompt: String,
    _command: String,
}

impl Game {
    pub fn new() -> Self {
        let display = Display::new();

        Self {
            display,
            _prompt: String::new(),
            _command: String::new(),
        }
    }

    pub fn run(&mut self) {
        // self.display.text_demo();
        self.test_texel_put();
        self.display.canvas.present();

        'running: loop {
            // print prompt

            // get user input
            for event in self.display.event_pump.poll_iter() {
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

    fn test_texel_put(&mut self) {
        let test_string = "Hello Texine!";
        for i in 0..test_string.len() {
            self.display.put_texel(
                test_string.chars().nth(i).unwrap(), 
                (i.try_into().unwrap(), 0)
            )
        }

    }
    
}
