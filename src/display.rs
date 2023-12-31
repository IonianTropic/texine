use core::panic;

use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::pixels::Color;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;



const FONT_SIZE: u16 = 30;
const PADDING: u32 = 20;
const REPRESENTATIVE_CHAR: char = 'A';

struct Sdl2Display {
    // Private SDL2-related fields
    canvas: Canvas<sdl2::video::Window>,
    texture_creator: TextureCreator<sdl2::video::WindowContext>,
    ttf_context: Sdl2TtfContext,
    event_pump: EventPump,
}

impl Sdl2Display {
    pub fn new(screen_size: (u32, u32), ttf_context: Sdl2TtfContext, font: &Font) -> Self {
        let (texel_width, texel_height) = TexelMap::calculate_texel_dimensions(font, REPRESENTATIVE_CHAR);
        
        let screen_width = PADDING*2 + screen_size.0*texel_width;
        let screen_height = PADDING*2 + screen_size.1*texel_height;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo", screen_width, screen_height)
            .position_centered()
            .build()
            .unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        Self {
            canvas,
            texture_creator,
            ttf_context,
            event_pump,
        }
    }



    pub fn copy_char(&mut self, character: char, target_rect: &Rect) {
        let font = self.ttf_context
            .load_font("./assets/fonts/PerfectDOSVGA437Win.ttf", FONT_SIZE)
            .unwrap();
        let surface = font
            .render_char(character)
            .blended(Color::RGB(255,255,255))
            .map_err(|e| e.to_string())
            .unwrap();

        let texture = self.texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        self.canvas.copy(&texture, None, *target_rect);
    }


}

// This is what the user will interact with
pub struct Display {
    sdl2display: Sdl2Display,
    pub texel_map: TexelMap,
    pub screen_size: (u32, u32),
    pub screen_mutable: bool,
    pub texel_size: u16,
    pub texel_size_mutable: bool,
}


impl Display {
    pub fn new(
        screen_size: (u32, u32), 
        screen_mutable: bool, 
        texel_size: u16, 
        texel_size_mutable: bool
    ) -> Self {
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context
        .load_font("./assets/fonts/PerfectDOSVGA437Win.ttf", texel_size)
        .unwrap();

        // FIXME: It's a miracle this wasn't throwing errors before
        // This is a big issue as a result of storing either the ttf_context or font in a field
        let sdl2display = Sdl2Display::new(screen_size, ttf_context, &font);

        let texel_map = TexelMap::new(screen_size, &font);

        Self {
            sdl2display,
            texel_map,
            screen_size,
            screen_mutable,
            texel_size,
            texel_size_mutable
        }
    }

    pub fn write_texel(&mut self, character: char, location: (usize, usize)) {
        self.texel_map.put_texel(character, location);
        self.sdl2display.copy_char(
            character, 
            self.texel_map.get_rect(location)
        );
    }

    // renders/copies all texels into the window
    pub fn update_all(&mut self, ) {
        panic!("Not yet implemented :(");
    }

    pub fn get_event_pump(&mut self) -> &mut EventPump {
        &mut self.sdl2display.event_pump
    }

    pub fn resize_window(&mut self, size: (u32,u32)) {
        if self.screen_mutable {
            // Change screen size in pixels
            // Do we then adjust the screen size in texels to fit, adjust the size of texels, or just leave it?
            panic!("Not yet implemented");
        }
    }
}

#[derive(Clone)]
struct Texel {
    character: char,

}

pub struct TexelMap {
    width: usize,
    height: usize,
    texels: Vec<Texel>,
    rects: Vec<Rect>,
}

impl TexelMap {
    pub fn new(screen_size: (u32, u32), font: &Font) -> Self {
        let width = screen_size.0 as usize;
        let height = screen_size.1 as usize;
        let num_texels = width * height;
        let texels = vec![Texel {character: ' '}; num_texels];

        let (texel_width, texel_height) = Self::calculate_texel_dimensions(font, REPRESENTATIVE_CHAR);
        let mut rects = Vec::with_capacity(width * height);

        for (idx, (x, y)) in itertools::iproduct!(0..width, 0..height).enumerate() {
            rects[idx] = Rect::new(
                x as i32 * texel_width as i32 + PADDING as i32,
                y as i32 * texel_height as i32 + PADDING as i32,
                texel_width,
                texel_height
            )
        }

        Self {
            width,
            height,
            texels,
            rects,
        }
    }

    pub fn calculate_texel_dimensions(font: &Font, representative_char: char) -> (u32, u32) {
            font.size_of_char(representative_char).unwrap()
        }

    pub fn get_texel(&self, location: (usize, usize)) -> &Texel {
        let (x, y) = location;
        let index = y * self.width + x;
        &self.texels[index]
    }

    pub fn get_rect(&self, location: (usize, usize)) -> &Rect {
        let (x, y) = location;
        let index = y * self.width + x;
        &self.rects[index]
    }

    pub fn put_texel(&mut self, character: char, location: (usize, usize)) {
        if location.0 < self.width && location.1 < self.height {
            let index = location.1 * self.width + location.0;
            self.texels[index].character = character;
        }
    }

    pub fn fill(&mut self, character: char) {
        for idx in 0..self.texels.len() {
            self.texels[idx].character = character;
        }
    }

    pub fn clear(&mut self) {
        self.fill(' ');
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        for (index, texel) in self.texels.iter().enumerate() {
            output.push(texel.character);

            if (index + 1) % self.width == 0 {
                output.push('\n');
            }
        }

        output
    }

    pub fn get_dirty_indices(&self) -> Vec<usize> {
        // Find which texels have been changed and not yet displayed
        panic!("Not yet implemented :(");
    }


}