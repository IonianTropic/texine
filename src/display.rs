use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::pixels::Color;
use sdl2::render::TextureQuery;
use sdl2::rect::Rect;

const FONT_SIZE: u16 = 20;
// FIXME: screen width and height should be dynamically set elsewhere
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
// Font is monospace, so this is the character used for different calculations like dimensions
const REPRESENTATIVE_CHAR: char = 'A';  

pub struct Display {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    ttf_context: Sdl2TtfContext,
    pub texel_map: TexelMap,
}

impl Display {
    pub fn new(texel_map: TexelMap) -> Self {
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
            texel_map,
        }
    }

    fn get_texel_size(&self) -> (u32, u32) {
        let font = self.ttf_context
            .load_font("./assets/fonts/PerfectDOSVGA437Win.ttf", FONT_SIZE)
            .unwrap();

        font.size_of_char(REPRESENTATIVE_CHAR).unwrap()
    }

    // Renders text as a surface and then converts it to a texture and copies it to the target_rect on display
    fn copy_text(&mut self, text: String, target_rect: Rect) {
        let font = self.ttf_context
            .load_font("./assets/fonts/PerfectDOSVGA437Win.ttf", FONT_SIZE)
            .unwrap();

        let surface = font
            .render(&text)
            .blended(Color::RGB(255,255,255))   // Abstract this
            .map_err(|e| e.to_string())
            .unwrap();

        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        self.canvas.copy(&texture, None, target_rect).unwrap();
    }

    // Updates entire screen in one go from the texel_map
    pub fn update_display(&mut self) {
        let text = self.texel_map.to_string();

        let (texel_width, texel_height) = self.get_texel_size();

        let map_width: u32 = self.texel_map.width.try_into().unwrap();
        let map_height: u32 = self.texel_map.height.try_into().unwrap();

        let target_rect = Rect::new(
            0,
            0,
            texel_width * map_width,
            texel_height * map_height
        );

        self.copy_text(text, target_rect);
        self.canvas.present();

    }


}

#[derive(Clone)]
struct Texel {
    character: char,
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

    pub fn put_texel(&mut self, character: char, location: (usize, usize)) {
        if location.0 < self.width && location.1 < self.height {
            let index = location.1 * self.width + location.0;
            self.texels[index].character = character;
        }
    }

    pub fn update_display(&mut self, ) {
        // Either update display here or have an equivalent method in Display.
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


}