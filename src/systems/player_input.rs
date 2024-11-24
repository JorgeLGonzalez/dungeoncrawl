use super::helpers::{PlayerAction, PlayerActionHelper};
use crate::prelude::*;

pub fn player_input(
    mut commands: Commands,
    mut attack_events: EventWriter<WantsToAttack>,
    mut move_events: EventWriter<WantsToMove>,
    player_query: Query<(Entity, &PointC), With<Player>>,
    enemy_query: Query<(Entity, &PointC), With<Enemy>>,
    key: Option<Res<VirtualKeyCode>>,
) {
    let helper =
        PlayerActionHelper::new(key.map(|k| k.as_ref().clone()), &player_query, &enemy_query);

    if let Some(action) = helper.determine_action() {
        match action {
            PlayerAction::ActivateItem(a) => {
                // commands.extend(a);
            }
            PlayerAction::Attack(a) => {
                attack_events.send(a);
            }
            PlayerAction::GetMagicItem => {}
            // PlayerAction::GetMagicItem => helper.pick_up_item(ecs, commands),
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
