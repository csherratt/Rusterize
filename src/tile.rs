use image::Rgb;

pub struct Tile {
    depth: [[f32; 32]; 32],
    color: [[Rgb<u8>; 32]; 32]
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            depth: [[1.; 32]; 32],
            color: [[Rgb([0, 0, 0]); 32]; 32]
        }
    }

    pub fn clear(&mut self) {
        self.depth = [[1.; 32]; 32];
        self.color = [[Rgb([0, 0, 0]); 32]; 32];
    }
}
