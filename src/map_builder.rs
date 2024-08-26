use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    fn fill(&mut self, tile : TileType){
        self.map.tiles.iter_mut().for_each(|t| *t = tile); 
    }
    fn build_random_room(&mut self, rng : &mut RandomNumberGenerator){
        while self.rooms.len() < NUM_ROOMS {
            let rect = Rect::with_size(rng.range(1, SCREEN_WIDTH -10),
                                       rng.range(1, SCREEN_HEIGHT - 10),
                                       rng.range(2, 10),
                                       rng.range(2, 10),);

        }

    }
}
