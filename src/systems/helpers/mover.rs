use crate::components::Name as EntityName;
use crate::prelude::*;

pub mod prelude {
    pub use super::FovQuery;
    pub use super::Mover;
}

pub type FovQuery<'w, 's, 'f, 'n, 'p> = Query<
    'w,
    's,
    (
        Entity,
        &'f FieldOfView,
        Option<&'n EntityName>,
        Option<&'p Player>,
    ),
>;

pub struct Mover {
    pub destination: Point,
    entity: Entity,
    fov: FieldOfView,
    is_player: bool,
    name: String,
}

impl Mover {
    pub fn new(fov_query: &FovQuery, movement: &WantsToMove) -> Self {
        let (_, fov, name, player) = fov_query.get(movement.entity).unwrap();

        Self {
            destination: movement.destination,
            entity: movement.entity,
            fov: fov.clone(),
            is_player: player.is_some(),
            name: name.map(|n| n.0.clone()).unwrap_or("Player".to_string()),
        }
    }

    pub fn do_move(&self, commands: &mut Commands) {
        commands
            .entity(self.entity)
            .insert(PointC(self.destination));
        commands.entity(self.entity).insert(self.fov.clone_dirty());

        println!("{} moves to {:?}", self.name, self.destination);
    }

    pub fn handle_player_move(&self, camera: &mut MainCamera, map: &mut Map) {
        if !self.is_player {
            return;
        }

        println!("\tMove camera to follow player and reveal more of the map.");
        self.fov.visible_tiles.iter().for_each(|pos| {
            map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
        });

        camera.on_player_move(self.destination);
    }

    /// Returns true when it is not the mover's turn. This happens because the
    /// movement system is executed in both the player's and monster's turn, which
    /// means the WantsToMove message is processed twice.
    pub fn out_of_turn(&self, turn: TurnState) -> bool {
        match turn {
            TurnState::MonsterTurn => self.is_player,
            TurnState::PlayerTurn => !self.is_player,
            _ => unreachable!(),
        }
    }
}
