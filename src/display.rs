use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::pixels::Color;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;

const FONT_SIZE: u16 = 20;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub struct Display {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    ttf_context: Sdl2TtfContext,
}

impl Display {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo", SCREEN_WIDTH, SCREEN_HEIGHT)
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
        }
    }

    // Location is coordinates in texels, starts at 0 and allows only positive integers.
    pub fn put_texel(&mut self, character: char, location: (u32, u32)) {

        let font = self.ttf_context
            .load_font("./assets/fonts/PerfectDOSVGA437Win.ttf", FONT_SIZE)
            .unwrap();

        let (texel_width, texel_height) = font.size_of_char(character).unwrap();
        // NOTE: Using ttf::GlyphMetrics might be a better choice here
        
        let num_texel_rows = SCREEN_HEIGHT/texel_height;
        let num_texel_cols = SCREEN_WIDTH/texel_width;

        // Check if location is valid
        if location.1 > num_texel_rows || location.0 > num_texel_cols {
            panic!("index out of bounds >:(")
        }

        // Create character texture (TEMPORARY)
        let surface = font
            .render_char(character)
            .blended(Color::RGB(255,255,255))   // Abstract this
            .map_err(|e| e.to_string())
            .unwrap();
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        // Create target rect
        let target_rect = Rect::new(
            (location.0 * texel_width).try_into().unwrap(),
            (location.1 * texel_height).try_into().unwrap(),
            texel_width,
            texel_height
        );
        // Copy character texture
        self.canvas.copy(&texture, None, target_rect).unwrap();

    }

    pub fn _text_demo(&mut self, text: String) {        
        // Create surface with desired text
        let font = self.ttf_context
            .load_font("./assets/fonts/PerfectDOSVGA437Win.ttf", FONT_SIZE)
            .unwrap();

        let surface = font
            .render(&text)
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