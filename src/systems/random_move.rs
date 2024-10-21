use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(MovingRandomly)]
#[read_component(Player)]
#[read_component(Point)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut rng = RandomNumberGenerator::new();
    let mut positions = <(Entity, &Point, &Health)>::query();

    <(Entity, &Point, &MovingRandomly)>::query()
        .iter(ecs)
        .for_each(|(entity, pos, _)| {
            let destination = determine_destination(&mut rng, *pos);
            let occupants: Vec<Occupant> = positions
                .iter(ecs)
                .filter(|p| destination_occupied(destination, p))
                .map(|(victim, _, _)| identify_occupant(ecs, *victim))
                .collect();

            if let Some(player_to_attack) = find_player_occupant(&occupants) {
                commands.push(((), WantsToAttack::new(*entity, player_to_attack)));
            } else if occupants.is_empty() {
                commands.push(((), WantsToMove::new(destination, *entity)));
            } else {
                println!(
                    "Monster blocked from moving by another monster at {:?}",
                    destination
                );
            }
        });
}

fn destination_occupied(destination: Point, (_, pos, _): &(&Entity, &Point, &Health)) -> bool {
    **pos == destination
}

fn determine_destination(rng: &mut RandomNumberGenerator, pos: Point) -> Point {
    let delta = match rng.range(0, 4) {
        0 => Point::new(-1, 0),
        1 => Point::new(1, 0),
        2 => Point::new(0, -1),
        _ => Point::new(0, 1),
    };

    delta + pos
}

fn find_player_occupant(occupants: &[Occupant]) -> Option<Entity> {
    occupants
        .iter()
        .filter_map(|o| match *o {
            Occupant::Player(p) => Some(p),
            _ => None,
        })
        .last()
}

fn identify_occupant(ecs: &SubWorld, occupant: Entity) -> Occupant {
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

enum Occupant {
    Player(Entity),
    FellowMonster,
}
