use crate::prelude::*;
use std::collections::HashMap;

pub enum ChaseAction {
    Attack(WantsToAttack),
    Move(WantsToMove),
}

impl ChaseAction {
    fn new_attack(attacker: Entity, player: Entity) -> Self {
        Self::Attack(WantsToAttack::new(attacker, player))
    }

    fn new_move(mover: Entity, destination: Point) -> Self {
        Self::Move(WantsToMove::new(mover, destination))
    }
}

pub struct ChaseActionDeterminer<'a> {
    dijkstra_map: DijkstraMap,
    enemy_positions: HashMap<Entity, Point>,
    map: &'a Map,
    player: PlayerInfo,
}

impl<'a> ChaseActionDeterminer<'a> {
    pub fn new(
        player_query: Query<(Entity, &PointC), With<Player>>,
        enemy_positions: Query<(Entity, &PointC), With<Enemy>>,
        map: &'a Map,
    ) -> Self {
        let enemy_positions = HashMap::from_iter(enemy_positions.iter().map(|(e, p)| (e, p.0)));
        let player = PlayerInfo::from_query(player_query);

        Self {
            dijkstra_map: create_dijkstra_map(player.pos, map),
            enemy_positions,
            map,
            player,
        }
    }

    pub fn determine(
        &mut self,
        mover: Entity,
        mover_pos: &PointC,
        fov: &FieldOfView,
    ) -> Option<ChaseAction> {
        if !fov.visible_tiles.contains(&self.player.pos) {
            return None;
        }

        self.determine_destination(mover_pos.0).map(|destination| {
            if destination == self.player.pos {
                ChaseAction::new_attack(mover, self.player.entity)
            } else {
                self.enemy_positions.insert(mover.clone(), destination);
                ChaseAction::new_move(mover, destination)
            }
        })
    }

    /// Move towards nearby player, or nearest exit if player is not nearby
    /// Block moves to enemy positions.
    /// See README.md#issue-monsters-able-to-move-on-top-of-each-other)
    fn determine_destination(&self, mover_pos: Point) -> Option<Point> {
        let distance = DistanceAlg::Pythagoras.distance2d(mover_pos, self.player.pos);
        // see p 315 for rationale for 1.2
        if distance > 1.2 {
            self.move_towards_player(mover_pos)
        } else {
            Some(self.player.pos)
        }
    }

    fn move_towards_player(&self, mover_pos: Point) -> Option<Point> {
        let idx = map_idx(mover_pos.x, mover_pos.y);

        DijkstraMap::find_lowest_exit(&self.dijkstra_map, idx, self.map)
            .map(|destination_idx| self.map.index_to_point2d(destination_idx))
            .filter(|pos| !self.position_taken(pos))
    }

    fn position_taken(&self, destination: &Point) -> bool {
        self.enemy_positions
            .values()
            .find(|&p| p == destination)
            .is_some()
    }
}

struct PlayerInfo {
    entity: Entity,
    pos: Point,
}

impl PlayerInfo {
    fn from_query(query: Query<(Entity, &PointC), With<Player>>) -> Self {
        let (entity, pos_c) = query.single();

        Self {
            entity,
            pos: pos_c.0,
        }
    }
}

fn create_dijkstra_map(player_pos: Point, map: &Map) -> DijkstraMap {
    let player_idx = map_idx(player_pos.x, player_pos.y);
    let search_targets = vec![player_idx];

    DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0)
}
