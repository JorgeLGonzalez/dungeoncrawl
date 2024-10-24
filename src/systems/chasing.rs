use crate::prelude::*;

#[system]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let player_pos = get_player_pos(ecs);
    let dijkstra_map = create_dijkstra_map(player_pos, map);

    let mut planned_moves: Vec<WantsToMove> = Vec::new();

    <(Entity, &Point, &ChasingPlayer)>::query()
        .iter(ecs)
        .map(|(entity, pos, _)| {
            determine_action(*entity, *pos, player_pos, &dijkstra_map, map, ecs)
        })
        .for_each(|a| match a {
            Action::Attack(a) => {
                commands.push(((), a));
            }
            Action::Move(m) => {
                if !will_be_occupied(&planned_moves, m) {
                    commands.push(((), m));
                    planned_moves.push(m);
                }
            }
            Action::None => (),
        });
}

fn create_dijkstra_map(player_pos: Point, map: &Map) -> DijkstraMap {
    let player_idx = map_idx(player_pos.x, player_pos.y);
    let search_targets = vec![player_idx];
    DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0)
}

fn determine_action(
    monster: Entity,
    monster_pos: Point,
    player_pos: Point,
    dijkstra_map: &DijkstraMap,
    map: &Map,
    ecs: &SubWorld,
) -> Action {
    let mut positions = <(Entity, &Point, &Health)>::query();

    let idx = map_idx(monster_pos.x, monster_pos.y);
    if let Some(destination) = DijkstraMap::find_lowest_exit(dijkstra_map, idx, map) {
        let distance = DistanceAlg::Pythagoras.distance2d(monster_pos, player_pos);
        // see p 315 for rationale for 1.2
        let destination = if distance > 1.2 {
            map.index_to_point2d(destination)
        } else {
            player_pos
        };

        let occupants: Vec<Occupant> = positions
            .iter(ecs)
            .filter(|o| occupied(destination, o))
            .map(|(victim, ..)| identify(ecs, *victim))
            .collect();

        if let Some(player_to_attack) = find_player(&occupants) {
            println!(
                "Monster {:?} attacks player {:?} at {:?}",
                monster, player_to_attack, destination
            );
            Action::Attack(WantsToAttack::new(monster, player_to_attack))
        } else if occupants.is_empty() {
            Action::Move(WantsToMove::new(monster, destination))
        } else {
            println!(
                "Monster {:?} unable to move to {:?} already occupied by a fellow monster",
                monster, destination
            );
            Action::None
        }
    } else {
        Action::None
    }
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

fn get_player_pos(ecs: &SubWorld) -> Point {
    let mut player = <(&Point, &Player)>::query();
    *player.iter(ecs).nth(0).unwrap().0
}

fn identify(ecs: &SubWorld, occupant: Entity) -> Occupant {
    if ecs
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

fn occupied(destination: Point, (_, pos, _): &(&Entity, &Point, &Health)) -> bool {
    destination == **pos
}

// see README.md#issue-monsters-able-to-move-on-top-of-each-other)
fn will_be_occupied(planned_moves: &[WantsToMove], this_move: WantsToMove) -> bool {
    let will_be_occupied = planned_moves
        .iter()
        .find(|pm| pm.destination == this_move.destination)
        .is_some();

    if will_be_occupied {
        println!(
            ">>>> Ignoring move to {:?} for {:?} already planned by another monster.",
            this_move.destination, this_move.entity
        )
    }

    will_be_occupied
}

enum Occupant {
    Player(Entity),
    FellowMonster,
}

enum Action {
    Attack(WantsToAttack),
    Move(WantsToMove),
    None,
}
