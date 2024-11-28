use super::helpers::Damager;
use crate::prelude::*;

pub fn combat(
    mut attack_events: EventReader<WantsToAttack>,
    mut health_query: Query<&mut Health>,
    mut commands: Commands,
    player_query: Query<&Player>,
    base_damage_query: Query<&Damage>,
    weapon_damage_query: Query<(&Carried, &Damage)>,
    turn: Res<TurnState>,
) {
    attack_events
        .iter()
        .map(|attack| Damager::new(attack, &player_query))
        .filter(|damager| !damager.out_of_turn(turn.to_owned()))
        .for_each(|damager| {
            damager
                .base_damage(&base_damage_query)
                .weapon_damage(&weapon_damage_query)
                .adjust_health(&mut health_query)
                .maybe_despawn(&mut commands);
        });
}
