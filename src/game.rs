use sdl2::{render::Canvas, video::Window, event::Event, keyboard::Keycode, EventPump};


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
        }
    }
}
