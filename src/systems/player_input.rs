use super::helpers::{PlayerAction, PlayerActionHelper};
use crate::prelude::*;

pub fn player_input(
    mut player_query: Query<&mut PointC, With<Player>>,
    (map, key, mut camera): (Res<Map>, Option<Res<VirtualKeyCode>>, ResMut<Camera>),
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
            let mut pos = player_query.single_mut();
            let destination = pos.0 + delta;
            if map.can_enter_tile(destination) {
                pos.0 = destination;
                camera.on_player_move(destination);
            }
        }
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
