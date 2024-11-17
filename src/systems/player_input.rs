use super::helpers::{PlayerAction, PlayerActionHelper};
use crate::prelude::*;

pub fn player_input(
    player_query: Query<(Entity, &PointC), With<Player>>,
    key: Option<Res<VirtualKeyCode>>,
    mut move_events: EventWriter<WantsToMove>,
    mut commands: Commands,
) {
    if let Some(key) = key.as_deref() {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        if delta.x != 0 || delta.y != 0 {
            let (player, pos) = player_query.single();
            let destination = pos.0 + delta;
            move_events.send(WantsToMove::new(player, destination));
        }

        commands.insert_resource(TurnState::PlayerTurn);
    }
    // if let Ok(mut pos) = player_query.get_single_mut() {
    //     let helper = PlayerActionHelper::new(key, pos.0);

    // if let Some(action) = helper.determine_action(ecs) {
    //     match action {
    //         PlayerAction::ActivateItem(a) => {
    //             // commands.extend(a);
    //         }
    //         PlayerAction::Attack(a) => {
    //             // commands.extend(a);
    //         }
    //         PlayerAction::GetMagicItem => {}
    //         // PlayerAction::GetMagicItem => helper.pick_up_item(ecs, commands),
    //         PlayerAction::Heal => (), // no longer in use
    //         // PlayerAction::Heal => helper.heal(ecs), // no longer in use
    //         PlayerAction::Move(m) => {}
    //         PlayerAction::ShowPlayerPosition => println!(">>>Player at {:?}", helper.pos),
    //         PlayerAction::Wait => (),
    //     };

    // *turn_state = TurnState::PlayerTurn;

    // };
    // }

    commands.remove_resource::<VirtualKeyCode>();
}

/*
In Bevy, keyboard input is handled differently. But here we are stull using bracket-lib.
See https://saveriomiroddi.github.io/learn_bevy_ecs_by_ripping_off/06_01/keyboard_input.html
 */
