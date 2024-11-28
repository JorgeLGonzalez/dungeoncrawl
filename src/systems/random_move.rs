use crate::prelude::*;

/// No longer in use since no monsters are set with this component.
/// And when ported to Bevy I didn't reimplement the attack mode
/// See https://github.com/64kramsystem/learn_bevy_ecs_by_ripping_off-code/blob/master/port/15_Loot_02_better_combat/src/systems/mod.rs
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
