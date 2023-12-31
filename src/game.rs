use crate::display::{Display, TexelMap};
use std::{thread, time};

use sdl2::{event::Event, keyboard::Keycode};


pub struct Game {
    display: Display,
    _prompt: String,
    _command: String,
}

impl Game {
    pub fn new() -> Self {
        let display = Display::new(50, 20);

        Self {
            display,
            _prompt: String::new(),
            _command: String::new(),
        }
    }

    pub fn run(&mut self) {
        // self.test_texel_display();
        self.display.texel_map.fill('A');
        self.display.update_all();


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

    fn test_texel_display(&mut self) {
        let text = "Hello world!";

        for (i, ch) in text.chars().enumerate() {
            self.display.texel_map.put_texel(ch, (i, i));
        }

        self.display.update_all();

    }

}
