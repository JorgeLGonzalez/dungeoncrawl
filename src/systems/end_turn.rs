use crate::prelude::*;

pub fn end_turn(
    mut commands: Commands,
    amulet_query: AmuletPosQuery,
    map: Res<Map>,
    player_query: PlayerQuery,
    turn_state: Res<TurnState>,
) {
    let info = PlayerInfo::new(&map, &player_query);

    let new_state = if info.health < 1 {
        TurnState::GameOver
    } else if info.tile == TileType::Exit {
        TurnState::NextLevel
    } else if info.pos == amulet_pos(&amulet_query) {
        TurnState::Victory
    } else {
        match *turn_state {
            TurnState::MonsterTurn => TurnState::AwaitingInput,
            TurnState::PlayerTurn => TurnState::MonsterTurn,
            _ => turn_state.clone(),
        }
    };

    commands.insert_resource(new_state);
}

struct PlayerInfo {
    health: i32,
    pos: Point,
    tile: TileType,
}

impl PlayerInfo {
    fn new(map: &Map, player_query: &PlayerQuery) -> Self {
        let (pos_c, health) = player_query.single();
        let pos = pos_c.0;

        Self {
            health: health.current,
            pos,
            tile: map.tiles[map.point2d_to_index(pos)],
        }
    }
}

type PlayerQuery<'w, 's, 'p, 'h> = Query<'w, 's, (&'p PointC, &'h Health), With<Player>>;

fn amulet_pos(query: &AmuletPosQuery) -> Point {
    query
        .iter()
        .nth(0)
        .map_or_else(|| Point::new(-1, -1), |p| p.0)
}

type AmuletPosQuery<'w, 's, 'p> = Query<'w, 's, &'p PointC, With<AmuletOfYala>>;
