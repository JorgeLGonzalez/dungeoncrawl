use super::helpers::{
    CarriedWeaponsQuery, EnemiesQuery, ItemsQuery, PlayerAction, PlayerActionHelper, PlayerQuery,
};
use crate::prelude::*;

pub fn player_input(
    mut commands: Commands,
    mut attack_events: EventWriter<WantsToAttack>,
    mut move_events: EventWriter<WantsToMove>,
    key: Option<Res<VirtualKeyCode>>,
    carried_weapons_query: CarriedWeaponsQuery,
    enemies_query: EnemiesQuery,
    items_query: ItemsQuery,
    player_query: PlayerQuery,
) {
    let key = key.map(|k| k.as_ref().clone());
    let helper = PlayerActionHelper::new(key, &player_query, &enemies_query);

    if let Some(action) = helper.determine_action() {
        match action {
            PlayerAction::ActivateItem(a) => {
                // commands.extend(a);
            }
            PlayerAction::Attack(a) => {
                attack_events.send(a);
            }
            PlayerAction::GetMagicItem => {
                helper.pick_up_item(&carried_weapons_query, &items_query, &mut commands)
            }
            PlayerAction::Heal => (), // no longer in use
            // PlayerAction::Heal => helper.heal(ecs), // no longer in use
            PlayerAction::Move(m) => {
                move_events.send(m);
            }
            PlayerAction::ShowPlayerPosition => println!(">>>Player at {:?}", helper.pos),
            PlayerAction::Wait => (),
        };

        commands.insert_resource(TurnState::PlayerTurn);
    }

    commands.remove_resource::<VirtualKeyCode>();
}

/*
In Bevy, keyboard input is handled differently. But here we are still using bracket-lib.
See https://saveriomiroddi.github.io/learn_bevy_ecs_by_ripping_off/06_01/keyboard_input.html
 */
