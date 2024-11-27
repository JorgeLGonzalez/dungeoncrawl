use crate::prelude::*;

pub fn random_move(
    mut movers: Query<(Entity, &PointC), With<MovingRandomly>>,
    mut move_events: EventWriter<WantsToMove>,
) {
    movers.iter_mut().for_each(|(mover, pos)| {
        let mut rng = RandomNumberGenerator::new();
        let delta = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        };

        let destination = pos.0 + delta;
        move_events.send(WantsToMove::new(mover, destination));
    });
}
