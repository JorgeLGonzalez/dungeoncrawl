use crate::components::Name as NameComponent;
use crate::prelude::*;

pub mod prelude {
    pub use super::CarriedItemsQuery;
    pub use super::EnemiesQuery;
    pub use super::ItemsQuery;
    pub use super::PlayerAction;
    pub use super::PlayerActionHelper;
    pub use super::PlayerQuery;
}

pub type CarriedItemsQuery<'world, 'state, 'n, 'c, 'wp> =
    Query<'world, 'state, (Entity, &'n NameComponent, &'c Carried, Option<&'wp Weapon>)>;
pub type EnemiesQuery<'w, 's, 'p> = Query<'w, 's, (Entity, &'p PointC), With<Enemy>>;
pub type ItemsQuery<'w, 's, 'n, 'wp, 'p> =
    Query<'w, 's, (Entity, &'n NameComponent, Option<&'wp Weapon>, &'p PointC), With<Item>>;
pub type PlayerQuery<'w, 's, 'p> = Query<'w, 's, (Entity, &'p PointC), With<Player>>;

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
        player_query: &PlayerQuery,
        enemies_query: &EnemiesQuery,
    ) -> Self {
        let (player, pos_c) = player_query.single();
        let pos = pos_c.0;
        let destination = key.and_then(move_delta).map(|delta| delta + pos);
        let attack = destination.and_then(|destination| {
            enemies_query
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

    pub fn determine_action(
        &self,
        carried_weapons_query: &CarriedItemsQuery,
    ) -> Option<PlayerAction> {
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
            VirtualKeyCode::Key1 => self.select_item(0, carried_weapons_query),
            VirtualKeyCode::Key2 => self.select_item(1, carried_weapons_query),
            VirtualKeyCode::Key3 => self.select_item(2, carried_weapons_query),
            VirtualKeyCode::Key4 => self.select_item(3, carried_weapons_query),
            VirtualKeyCode::Key5 => self.select_item(4, carried_weapons_query),
            VirtualKeyCode::Key6 => self.select_item(5, carried_weapons_query),
            VirtualKeyCode::Key7 => self.select_item(6, carried_weapons_query),
            VirtualKeyCode::Key8 => self.select_item(7, carried_weapons_query),
            VirtualKeyCode::Key9 => self.select_item(8, carried_weapons_query),
            // _ => Action::Heal,
            _ => Some(PlayerAction::Wait),
        }
    }

    pub fn pick_up_item(
        &self,
        carried_weapons_query: &CarriedItemsQuery,
        items_query: &ItemsQuery,
        commands: &mut Commands,
    ) {
        if let Some((item, name, weapon)) = items_query
            .iter()
            .find(|(.., item_pos)| item_pos.0 == self.pos)
            .map(|(item, name, weapon, ..)| (item, name.0.as_str(), weapon))
        {
            if weapon.is_some() {
                self.maybe_replace_weapon(commands, name, carried_weapons_query);
            } else {
                println!("Player picks up {}", name);
            }

            commands.entity(item).insert(Carried(self.player));
            commands.entity(item).remove::<PointC>();
        }
    }

    fn maybe_replace_weapon(
        &self,
        commands: &mut Commands,
        picked_up_weapon: &str,
        carried_weapons_query: &CarriedItemsQuery,
    ) {
        if let Some((weapon, replaced)) = carried_weapons_query
            .iter()
            .find(|(.., carried, weapon)| carried.0 == self.player && weapon.is_some())
            .map(|(weapon, name, ..)| (weapon, name.0.as_str()))
        {
            commands.entity(weapon).despawn();
            println!("Player picks up {picked_up_weapon} weapon, replacing {replaced}");
        } else {
            println!("Player picks up {picked_up_weapon} weapon")
        }
    }

    fn select_item(&self, n: usize, items_query: &CarriedItemsQuery) -> Option<PlayerAction> {
        items_query
            .iter()
            .filter(|(.., carried, _)| carried.0 == self.player)
            .nth(n)
            .map(|(item, ..)| ActivateItem::new(item, self.player))
            .map(PlayerAction::ActivateItem)
    }
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
    ActivateItem(ActivateItem),
    Attack(WantsToAttack),
    GetMagicItem,
    Move(WantsToMove),
    ShowPlayerPosition,
    Wait,
}
