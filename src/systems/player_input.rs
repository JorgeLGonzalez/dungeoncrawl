use super::helpers::{PlayerAction, PlayerActionHelper};
use crate::prelude::*;

#[system]
#[read_component(Carried)]
#[read_component(Enemy)]
#[read_component(Item)]
#[read_component(Player)]
#[read_component(Point)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let helper = PlayerActionHelper::new(*key, ecs);
    let action = helper.determine_action(ecs);
    let take_turn = action != PlayerAction::None;

    match action {
        PlayerAction::ActivateItem(a) => {
            commands.extend(a);
        }
        PlayerAction::Attack(a) => {
            commands.extend(a);
        }
        PlayerAction::GetMagicItem => helper.pick_up_item(ecs, commands),
        PlayerAction::Heal => helper.heal(ecs), // no longer in use
        PlayerAction::Move(m) => {
            commands.extend(m);
        }
        PlayerAction::None => (),
        PlayerAction::ShowPlayerPosition => println!(">>>Player at {:?}", helper.pos),
        PlayerAction::Wait => (),
    };

    if take_turn {
        *turn_state = TurnState::PlayerTurn;
    }
}
