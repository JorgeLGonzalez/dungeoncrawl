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

            // TODO fn that returns attacks on player that we then push to commands
            occupants
                .iter()
                .filter_map(|o| match *o {
                    Occupant::Player(p) => Some(WantsToAttack::new(*entity, p)),
                    _ => None,
                })
                .for_each(|attack| {
                    commands.push(((), attack));
                });

            if occupants.is_empty() {
                commands.push(((), WantsToMove::new(destination, *entity)));
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
