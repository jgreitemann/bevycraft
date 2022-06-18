use crate::prelude::*;
use itertools::Itertools;

use super::MapArchitect;
use crate::map::map_builder::MapBuilder;
use rand::{rngs::ThreadRng, thread_rng, Rng};

#[derive(Default)]
pub struct CellularAutomataArchitect {}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut ThreadRng, mb: &mut MapBuilder) {
        mb.map_data.fill_with(|| {
            if rng.gen_range(0..100) > 55 {
                TileType::Floor
            } else {
                TileType::Wall
            }
        });
    }

    fn count_neighbors(&self, p: &Point, mb: &MapBuilder) -> usize {
        Itertools::cartesian_product(-1..=1, -1..=1)
            .filter(|&(dx, dy)| dx != 0 || dy != 0)
            .map(|(dx, dy)| Point::new(dx, dy))
            .filter_map(|delta| mb.try_idx(*p + delta))
            .filter(|&idx| mb.map_data[idx] == TileType::Wall)
            .count()
    }

    fn iteration(&mut self, mb: &mut MapBuilder) {
        let mut new_tiles = mb.map_data.clone();

        for (idx, new_tile) in new_tiles.iter_mut().enumerate() {
            *new_tile = match self.count_neighbors(&mb.index_to_point2d(idx), &mb) {
                0 => TileType::Wall,
                n if (1..=4).contains(&n) => TileType::Floor,
                _ => TileType::Wall,
            };
        }

        mb.map_data = new_tiles;
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn architect(&mut self) -> MapBuilder {
        let mut mb = MapBuilder::default();

        let mut rng = thread_rng();
        self.random_noise_map(&mut rng, &mut mb);
        for _ in 0..10 {
            self.iteration(&mut mb);
        }

        mb
    }
}
