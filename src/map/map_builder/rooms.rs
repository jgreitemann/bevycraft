use super::*;
use crate::prelude::*;
use rand::prelude::*;

const NUM_ROOMS: usize = 20;

#[derive(Default)]
pub struct RoomsArchitect {
    rooms: Vec<bracket_geometry::prelude::Rect>,
}

impl RoomsArchitect {
    fn build_random_rooms(&mut self, mb: &mut MapBuilder, rng: &mut ThreadRng) {
        while self.rooms.len() < NUM_ROOMS {
            let room = bracket_geometry::prelude::Rect::with_size(
                rng.gen_range(1..MAP_WIDTH - 10),
                rng.gen_range(1..MAP_HEIGHT - 10),
                rng.gen_range(2..10),
                rng.gen_range(2..10),
            );

            let overlap = self.rooms.iter().any(|r| r.intersect(&room));

            if !overlap {
                room.for_each(|p| {
                    let idx = mb.point2d_to_index(p);
                    mb.map_data[idx] = TileType::Floor;
                });
                self.rooms.push(room);
            }
        }
    }

    fn build_corridors(&mut self, mb: &mut MapBuilder, rng: &mut ThreadRng) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        use itertools::Itertools;
        for (prev_room, new_room) in rooms.iter().tuple_windows() {
            let prev = prev_room.center();
            let new = new_room.center();
            if rng.gen_range(0..2) == 1 {
                mb.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                mb.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                mb.apply_vertical_tunnel(prev.y, new.y, prev.x);
                mb.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}

impl MapArchitect for RoomsArchitect {
    fn architect(&mut self) -> MapBuilder {
        let mut mb = MapBuilder::default();

        let mut rng = thread_rng();
        self.build_random_rooms(&mut mb, &mut rng);
        self.build_corridors(&mut mb, &mut rng);
        mb.player_start = self.rooms[0].center().into();
        mb.amulet_start = mb.find_most_distant().into();

        mb.spawn_locations = self
            .rooms
            .iter()
            .skip(1)
            .map(|rect| rect.center().into())
            .collect();

        mb
    }
}
