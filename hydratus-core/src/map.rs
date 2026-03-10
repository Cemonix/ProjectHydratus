#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Empty,
    Obstacle,
    Tower,
    Unit,
    Building
}

impl Into<&'static str> for Tile {
    fn into(self) -> &'static str {
        match self {
            Tile::Empty => "Empty",
            Tile::Obstacle => "Obstacle",
            Tile::Tower => "Tower",
            Tile::Unit => "Unit",
            Tile::Building => "Building"
        }
    }
}

pub struct Map {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
    tile_size: u8
}

impl Map {
    pub fn new(width: u32, height: u32, tile_size: u8) -> Self {
        Map {
            width,
            height,
            tiles: Vec::new(),
            tile_size
        }
    }
}