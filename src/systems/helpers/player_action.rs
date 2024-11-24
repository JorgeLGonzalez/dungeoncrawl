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
        carried_weapons_query: &Query<(Entity, &NameComponent, &Carried), With<Weapon>>,
        items_query: &Query<(Entity, &NameComponent, Option<&Weapon>, &PointC), With<Item>>,
        commands: &mut Commands,
    ) {
        if let Some((item, name, weapon)) = items_query
            .iter()
            .find(|(.., item_pos)| item_pos.0 == self.pos)
            .map(|(item, name, weapon, ..)| (item, name.0.as_str(), weapon))
        {
            if weapon.is_some() {
                self.discard_replaced_weapon(carried_weapons_query, commands);
                println!("Player picks up {} weapon", name)
            } else {
                println!("Player picks up {}", name);
            }

            commands.entity(item).insert(Carried(self.player));
            commands.entity(item).remove::<PointC>();
        }
    }

    fn discard_replaced_weapon(
        &self,
        carried_weapons_query: &Query<(Entity, &NameComponent, &Carried), With<Weapon>>,
        commands: &mut Commands,
    ) {
        if let Some((weapon, name)) = carried_weapons_query
            .iter()
            .find(|(.., carried)| carried.0 == self.player)
            .map(|(weapon, name, ..)| (weapon, name.0.as_str()))
        {
            commands.entity(weapon).despawn();
            println!("Player drops {}", name);
        }
    }

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
