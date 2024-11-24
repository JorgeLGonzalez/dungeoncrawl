use crate::components::Name as NameComponent;
use crate::prelude::*;

pub struct PlayerActionHelper {
    attack: Option<WantsToAttack>,
    destination: Option<Point>,
    key: Option<VirtualKeyCode>,
    pub player: Entity,
    pub pos: Point,
}

impl PlayerActionHelper {
    pub fn new(
        key: Option<VirtualKeyCode>,
        player_query: &Query<(Entity, &PointC), With<Player>>,
        enemy_query: &Query<(Entity, &PointC), With<Enemy>>,
    ) -> Self {
        let (player, pos_c) = player_query.single();
        let pos = pos_c.0;
        let destination = key.and_then(move_delta).map(|delta| delta + pos);
        let attack = destination.and_then(|destination| {
            enemy_query
                .iter()
                .find(|(_, pos)| pos.0 == destination)
                .map(|(entity, _)| WantsToAttack::new(player, entity))
        });

        Self {
            attack,
            destination,
            key,
            player,
            pos,
        }
    }

    pub fn determine_action(&self) -> Option<PlayerAction> {
        if self.key.is_none() {
            return None;
        }

        if self.attack.is_some() {
            return self.attack.map(|a| PlayerAction::Attack(a));
        }

        if let Some(destination) = self.destination {
            return Some(PlayerAction::Move(WantsToMove::new(
                self.player,
                destination,
            )));
        }

        let key = self.key.unwrap();
        match key {
            VirtualKeyCode::G => Some(PlayerAction::GetMagicItem),
            VirtualKeyCode::P => Some(PlayerAction::ShowPlayerPosition),
            // VirtualKeyCode::Key1 => self.select_item(0, ecs),
            // VirtualKeyCode::Key2 => self.select_item(1, ecs),
            // VirtualKeyCode::Key3 => self.select_item(2, ecs),
            // VirtualKeyCode::Key4 => self.select_item(3, ecs),
            // VirtualKeyCode::Key5 => self.select_item(4, ecs),
            // VirtualKeyCode::Key6 => self.select_item(5, ecs),
            // VirtualKeyCode::Key7 => self.select_item(6, ecs),
            // VirtualKeyCode::Key8 => self.select_item(7, ecs),
            // VirtualKeyCode::Key9 => self.select_item(8, ecs),
            // _ => Action::Heal,
            _ => Some(PlayerAction::Wait),
        }
    }

    // pub fn heal(&self, ecs: &mut SubWorld) {
    //     if let Ok(health) = ecs
    //         .entry_mut(self.player)
    //         .unwrap()
    //         .get_component_mut::<Health>()
    //     {
    //         if health.current < health.max {
    //             health.current += 1;
    //             println!("Healed to {}", health.current);
    //         }
    //     }
    // }

    pub fn pick_up_item(
        &self,
        items_query: &Query<(Entity, &NameComponent, &Item, &PointC)>,
        commands: &mut Commands,
    ) {
        if let Some((item, name, pos)) = items_query
            .iter()
            .find(|(.., item_pos)| item_pos.0 == self.pos)
            .map(|(item, name, _, pos)| (item, name, pos.0))
        {
            println!("Picked up {:?}", name.0);
            commands.entity(item).remove::<PointC>();
            commands.entity(item).insert(Carried(self.player));
        }

        //     if is_weapon(item, ecs) {
        //         self.discard_replaced_weapon(commands, ecs);
        //         println!("Player picks up {} weapon", name.0)
        //     } else {
        //         println!("Player picks up {}", name.0);
        //     }
        // });
    }

    // fn discard_replaced_weapon(&self, commands: &mut CommandBuffer, ecs: &SubWorld) {
    //     <(Entity, &Carried, &Weapon, &Name)>::query()
    //         .iter(ecs)
    //         .filter(|(_, carried_weapon, ..)| carried_weapon.0 == self.player)
    //         .for_each(|(&weapon, .., name)| {
    //             commands.remove(weapon);
    //             println!("Player drops {}", name.0);
    //         });
    // }

    // fn select_item(&self, n: usize, ecs: &SubWorld) -> Option<PlayerAction> {
    //     <(Entity, &Item, &Carried)>::query()
    //         .iter(ecs)
    //         .filter(|(_, _, carried)| carried.0 == self.player)
    //         .enumerate()
    //         .filter(|(item_count, _)| *item_count == n)
    //         .find_map(|(_, (&item_entity, ..))| Some(ActivateItem::new(item_entity, self.player)))
    //         .map(|a| vec![((), a)])
    //         .map(|v| PlayerAction::ActivateItem(v))
    // }
}

// fn is_weapon(item: Entity, ecs: &SubWorld) -> bool {
//     ecs.entry_ref(item)
//         .map_or(false, |e| e.get_component::<Weapon>().is_ok())
// }

fn move_delta(key: VirtualKeyCode) -> Option<Point> {
    match key {
        VirtualKeyCode::Left => Some(Point::new(-1, 0)),
        VirtualKeyCode::Right => Some(Point::new(1, 0)),
        VirtualKeyCode::Up => Some(Point::new(0, -1)),
        VirtualKeyCode::Down => Some(Point::new(0, 1)),
        _ => None,
    }
}

#[derive(PartialEq)]
pub enum PlayerAction {
    ActivateItem(ActivateItemCommandVec),
    Attack(WantsToAttack),
    GetMagicItem,
    #[allow(dead_code)]
    Heal,
    Move(WantsToMove),
    ShowPlayerPosition,
    Wait,
}

type ActivateItemCommandVec = Vec<((), ActivateItem)>;
