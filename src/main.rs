use game::Game;
use display::TexelMap;

mod game;
mod display;

fn main() {
    Game::new().run();
    // let width = 4;
    // let height = 5;
    // let mut texel_map = TexelMap::new(width, height);

    // let mut character = 'A' as u32;
    // for y in 0..height {
    //     for x in 0..width {
    //         let ch = char::from_u32(character).unwrap();
    //         texel_map.put_texel(ch, (x, y));
    //         character += 1;
    //     }
    // }

    // println!("{}", texel_map.to_string());

}
