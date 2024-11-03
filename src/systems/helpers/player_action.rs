use crate::prelude::*;

pub struct PlayerActionHelper {
    attacks: AttackCommandVec,
    destination: Option<Point>,
    key: Option<VirtualKeyCode>,
    pub player: Entity,
    pub pos: Point,
}

impl PlayerActionHelper {
    pub fn new(key: Option<VirtualKeyCode>, ecs: &SubWorld) -> Self {
        let (player, pos) = get_player_info(ecs);
        let mut determiner = Self {
            attacks: Vec::new(),
            destination: key.and_then(delta).map(|delta| delta + pos),
            key,
            player,
            pos,
        };

        determiner.attacks = determiner.gather_attacks(ecs);

        determiner
    }

    pub fn determine_action(&self, ecs: &SubWorld) -> Option<PlayerAction> {
        if self.key.is_none() {
            return None;
        }

        if !self.attacks.is_empty() {
            return Some(PlayerAction::Attack(self.attacks.clone()));
        }

        if let Some(destination) = self.destination {
            return Some(PlayerAction::Move(vec![(
                (),
                WantsToMove::new(self.player, destination),
            )]));
        }

        let key = self.key.unwrap();
        match key {
            VirtualKeyCode::G => Some(PlayerAction::GetMagicItem),
            VirtualKeyCode::P => Some(PlayerAction::ShowPlayerPosition),
            VirtualKeyCode::Key1 => self.select_item(0, ecs),
            VirtualKeyCode::Key2 => self.select_item(1, ecs),
            VirtualKeyCode::Key3 => self.select_item(2, ecs),
            VirtualKeyCode::Key4 => self.select_item(3, ecs),
            VirtualKeyCode::Key5 => self.select_item(4, ecs),
            VirtualKeyCode::Key6 => self.select_item(5, ecs),
            VirtualKeyCode::Key7 => self.select_item(6, ecs),
            VirtualKeyCode::Key8 => self.select_item(7, ecs),
            VirtualKeyCode::Key9 => self.select_item(8, ecs),
            // _ => Action::Heal,
            _ => Some(PlayerAction::Wait),
        }
    }

    pub fn heal(&self, ecs: &mut SubWorld) {
        if let Ok(health) = ecs
            .entry_mut(self.player)
            .unwrap()
            .get_component_mut::<Health>()
        {
            if health.current < health.max {
                health.current += 1;
                println!("Healed to {}", health.current);
            }
        }
    }

    pub fn pick_up_item(&self, ecs: &mut SubWorld, commands: &mut CommandBuffer) {
        <(Entity, &Item, &Point)>::query()
            .iter(ecs)
            .filter(|(.., &item_pos)| item_pos == self.pos)
            .for_each(|(entity, ..)| {
                commands.remove_component::<Point>(*entity);
                commands.add_component(*entity, Carried(self.player));
            });
    }

    fn gather_attacks(&self, ecs: &SubWorld) -> AttackCommandVec {
        self.destination
            .map(|destination| {
                <(Entity, &Point)>::query()
                    .filter(component::<Enemy>())
                    .iter(ecs)
                    .filter(|(_, pos)| **pos == destination)
                    .map(|(entity, _)| ((), WantsToAttack::new(self.player, *entity)))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn select_item(&self, n: usize, ecs: &SubWorld) -> Option<PlayerAction> {
        <(Entity, &Item, &Carried)>::query()
            .iter(ecs)
            .filter(|(_, _, carried)| carried.0 == self.player)
            .enumerate()
            .filter(|(item_count, _)| *item_count == n)
            .find_map(|(_, (&item_entity, ..))| Some(ActivateItem::new(item_entity, self.player)))
            .map(|a| vec![((), a)])
            .map(|v| PlayerAction::ActivateItem(v))
    }
}

fn delta(key: VirtualKeyCode) -> Option<Point> {
    match key {
        VirtualKeyCode::Left => Some(Point::new(-1, 0)),
        VirtualKeyCode::Right => Some(Point::new(1, 0)),
        VirtualKeyCode::Up => Some(Point::new(0, -1)),
        VirtualKeyCode::Down => Some(Point::new(0, 1)),
        _ => None,
    }
}

fn get_player_info(ecs: &SubWorld) -> (Entity, Point) {
    <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .map(|(player, pos)| (*player, *pos))
        .unwrap()
}

#[derive(PartialEq)]
pub enum PlayerAction {
    ActivateItem(ActivateItemCommandVec),
    Attack(AttackCommandVec),
    GetMagicItem,
    #[allow(dead_code)]
    Heal,
    Move(MoveCommandVec),
    ShowPlayerPosition,
    Wait,
}

type ActivateItemCommandVec = Vec<((), ActivateItem)>;
type AttackCommandVec = Vec<((), WantsToAttack)>;
type MoveCommandVec = Vec<((), WantsToMove)>;
