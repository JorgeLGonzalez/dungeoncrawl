use super::get_player_pos;
use crate::prelude::*;

pub enum ChaseAction {
    Attack(WantsToAttack),
    Move(WantsToMove),
}

pub struct ChaseActionDeterminer<'a> {
    dijkstra_map: DijkstraMap,
    ecs: &'a SubWorld<'a>,
    map: &'a Map,
    planned_moves: Vec<WantsToMove>,
    player_pos: Point,
}

impl<'a> ChaseActionDeterminer<'a> {
    pub fn new(ecs: &'a SubWorld, map: &'a Map) -> Self {
        let player_pos = get_player_pos(ecs);

        Self {
            dijkstra_map: create_dijkstra_map(player_pos, map),
            ecs,
            map,
            planned_moves: Vec::new(),
            player_pos,
        }
    }

    pub fn determine(
        &mut self,
        monster: Entity,
        monster_pos: Point,
        fov: &FieldOfView,
    ) -> Option<ChaseAction> {
        if !fov.visible_tiles.contains(&self.player_pos) {
            return None;
        }

        let mut positions = <(Entity, &Point, &Health)>::query();

        let idx = map_idx(monster_pos.x, monster_pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&self.dijkstra_map, idx, self.map)
        {
            let distance = DistanceAlg::Pythagoras.distance2d(monster_pos, self.player_pos);
            // see p 315 for rationale for 1.2
            let destination = if distance > 1.2 {
                self.map.index_to_point2d(destination)
            } else {
                self.player_pos
            };

            let occupants: Vec<Occupant> = positions
                .iter(self.ecs)
                .filter(|o| occupied(destination, o))
                .map(|(victim, ..)| self.identify(*victim))
                .collect();

            if let Some(player_to_attack) = find_player(&occupants) {
                println!(
                    "Monster {:?} attacks player {:?} at {:?}",
                    monster, player_to_attack, destination
                );
                Some(ChaseAction::Attack(WantsToAttack::new(
                    monster,
                    player_to_attack,
                )))
            } else if self.will_be_occupied(monster, destination) {
                None
            } else if occupants.is_empty() {
                let this_move = WantsToMove::new(monster, destination);
                self.planned_moves.push(this_move);
                Some(ChaseAction::Move(this_move))
            } else {
                println!(
                    "Monster {:?} unable to move to {:?} already occupied by a fellow monster",
                    monster, destination
                );
                None
            }
        } else {
            None
        }
    }

    fn identify(&self, occupant: Entity) -> Occupant {
        if self
            .ecs
            .entry_ref(occupant)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            Occupant::Player(occupant)
        } else {
            Occupant::FellowMonster
        }
    }

    // see README.md#issue-monsters-able-to-move-on-top-of-each-other)
    fn will_be_occupied(&self, monster: Entity, destination: Point) -> bool {
        let will_be_occupied = self
            .planned_moves
            .iter()
            .find(|pm| pm.destination == destination)
            .is_some();

        if will_be_occupied {
            println!(
                ">>>> Ignoring move to {:?} for {:?} already planned by another monster.",
                destination, monster
            )
        }

        will_be_occupied
    }
}

fn create_dijkstra_map(player_pos: Point, map: &Map) -> DijkstraMap {
    let player_idx = map_idx(player_pos.x, player_pos.y);
    let search_targets = vec![player_idx];
    DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0)
}

fn find_player(occupants: &[Occupant]) -> Option<Entity> {
    occupants
        .iter()
        .filter_map(|o| match *o {
            Occupant::Player(p) => Some(p),
            _ => None,
        })
        .last()
}

fn occupied(destination: Point, (_, pos, _): &(&Entity, &Point, &Health)) -> bool {
    destination == **pos
}

enum Occupant {
    Player(Entity),
    FellowMonster,
}
