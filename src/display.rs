use core::panic;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::pixels::Color;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;

const FONT_SIZE: u16 = 30;
const PADDING: u32 = 20;
// Font is monospace, so this is the character used for different calculations like dimensions
const REPRESENTATIVE_CHAR: char = 'A';  

pub struct Display {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    ttf_context: Sdl2TtfContext,
    pub texel_size: (u32, u32),
    pub texel_map: TexelMap,
    pub screen_size: (u32, u32),
}

impl Display {
    // Width and height are in texels.
    pub fn new(width: u32, height: u32) -> Self {
        
        let ttf_context = sdl2::ttf::init().unwrap();
        let (texel_width, texel_height) = Self::get_texel_size(&ttf_context);
        
        let screen_width = PADDING*2 + width*texel_width;
        let screen_height = PADDING*2 + height*texel_height;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo", screen_width, screen_height)
            .position_centered()
            .build()
            .unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.clear();

        let texel_map = TexelMap::new(width as usize, height as usize);

        Self {
            canvas,
            event_pump,
            ttf_context,
            texel_size: (texel_width, texel_height),
            texel_map,
            screen_size: (width, height),
        }
    }

    fn get_texel_size(ttf_context: &Sdl2TtfContext) -> (u32, u32) {
        let font = ttf_context
            .load_font("./assets/fonts/PerfectDOSVGA437Win.ttf", FONT_SIZE)
            .unwrap();

        font.size_of_char(REPRESENTATIVE_CHAR).unwrap()
    }

    // Renders text as a surface and then converts it to a texture and copies it to the target_rect on display
    fn copy_char(&mut self, text: char, target_rect: Rect) {
        let font = self.ttf_context
            .load_font("./assets/fonts/PerfectDOSVGA437Win.ttf", FONT_SIZE)
            .unwrap();

        let surface = font
            .render_char(text)
            .blended(Color::RGB(255,255,255))   // Abstract this
            .map_err(|e| e.to_string())
            .unwrap();

        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        // let TextureQuery {width, height, ..} = texture.query();
        // println!("Width: {}. Height: {}", width, height); 

        self.canvas.copy(&texture, None, target_rect).unwrap();
    }

    // Updates entire screen in one go from the texel_map
    // FIXME: Slow
    pub fn update_all(&mut self) {
        self.canvas.clear();

        for y in 0..self.texel_map.height {
            for x in 0..self.texel_map.width {
                let target_rect = self.get_rect_from_texel(x, y);
                let texel = self.texel_map.get_texel(x, y);
                let texel_char = texel.character;

                self.copy_char(texel_char, target_rect);
                // self.canvas.present();

            }
        }
        self.canvas.present();
    }

    pub fn update_indices(&mut self, indices: Vec<usize>) {
        // TODO: test me
        // TODO: Implement dirty rectangles/flags to generate indices
        // similar to update_all, except update only the given indices
        for idx in indices.iter() {
            if *idx < self.texel_map.texels.len() {
                // Because idx = y*width + x:
                let x = idx % self.texel_map.width;
                let y = (idx - x) / self.texel_map.width;

                let target_rect = self.get_rect_from_texel(x, y);
                let texel = self.texel_map.get_texel(x, y);

                self.copy_char(texel.character, target_rect);
            }
        }
        self.canvas.present();
    }

    fn get_rect_from_texel(&mut self, x: usize, y: usize) -> Rect {
        let (texel_width, texel_height) = self.texel_size;

        Rect::new(
            x as i32 * texel_width as i32 + PADDING as i32,
            y as i32 * texel_height as i32 + PADDING as i32,
            texel_width,
            texel_height
        )
    }
}

#[derive(Clone)]
pub struct Texel {
    character: char,
    // dirty: bool, //FIXME: 
    // color: Color,
    // style: ,
    // etc
}

pub struct TexelMap {
    width: usize,
    height: usize,
    texels: Vec<Texel>,
}

impl TexelMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut texels = vec![Texel {character: ' '}; width * height];

        Self {
            width,
            height,
            texels,
        }
    }

    pub fn get_texel(&self, x: usize, y: usize) -> &Texel {
        let index = y * self.width + x;
        &self.texels[index]
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
        panic!("Not yet implemented >:(");
    }


}