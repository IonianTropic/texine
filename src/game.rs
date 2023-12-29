use crate::display::Display;
use std::{thread, time};

use sdl2::{event::Event, keyboard::Keycode};


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
        // self.display._text_demo();
        // self._test_texel_put();
        // self.display.canvas.present();
        self.texel_print("Hello world!");

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

    fn _test_texel_put(&mut self) {
        let test_string = "Hello Texine!";
        for i in 0..test_string.len() {
            self.display.put_texel(
                test_string.chars().nth(i).unwrap(), 
                (i.try_into().unwrap(), 0)
            )
        }
    }

    fn texel_print(&mut self, text: &str) {
        self.display.canvas.clear();
        for (character, i) in text.chars().zip(0..text.len()) {
            self.display.put_texel(character, (i.try_into().unwrap(), 0));
            self.display.canvas.present();

            thread::sleep(time::Duration::from_millis(50));
        }
    }
    
}
