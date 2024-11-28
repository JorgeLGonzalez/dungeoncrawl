use crate::prelude::*;

pub struct Damager {
    attacker: Entity,
    damage: i32,
    killed: bool,
    player_victim: bool,
    victim: Entity,
}

impl Damager {
    pub fn new(attack: &WantsToAttack, player_query: &Query<&Player>) -> Self {
        Self {
            attacker: attack.attacker,
            damage: 0,
            killed: false,
            player_victim: player_query.get(attack.victim).is_ok(),
            victim: attack.victim,
        }
    }

    pub fn adjust_health(mut self, health_query: &mut Query<&mut Health>) -> Self {
        if let Ok(mut health) = health_query.get_mut(self.victim) {
            health.current -= self.damage;
            self.killed = health.current < 1;
            self.log(health.current);
        }

        self
    }

    pub fn base_damage(self, base_damage_query: &Query<&Damage>) -> Self {
        let damage = base_damage_query
            .get(self.victim)
            .map_or(0, |damage| damage.0);

        Self {
            damage: self.damage + damage,
            ..self
        }
    }

    pub fn maybe_despawn(&self, commands: &mut Commands) {
        if self.killed && !self.player_victim {
            commands.entity(self.victim).despawn();
        }
    }

    /// True when attack is out of turn. This can happen because the combat system
    /// executes on both the player and monster turns so a player attack event
    /// can be processed in both the player's and monster's turn.
    pub fn out_of_turn(&self, turn: TurnState) -> bool {
        match turn {
            TurnState::MonsterTurn => !self.player_victim,
            TurnState::PlayerTurn => self.player_victim,
            _ => unreachable!(),
        }
    }

    pub fn weapon_damage(self, weapon_damage_query: &Query<(&Carried, &Damage)>) -> Self {
        let damage: i32 = weapon_damage_query
            .iter()
            .filter_map(|(carried, damage)| (carried.0 == self.attacker).then_some(damage.0))
            .sum();

        Self {
            damage: self.damage + damage,
            ..self
        }
    }

    fn log(&self, health: i32) {
        let attacker = if self.player_victim {
            "Monster"
        } else {
            "Player"
        };
        let damage = self.damage;
        let victim = if self.player_victim {
            "Player"
        } else {
            "Monster"
        };
        let status = if self.killed {
            "dead".to_string()
        } else {
            format!("with {health} health points")
        };

        println!("{attacker} inflicts {damage} points of damage leaving {victim} {status}",)
    }
}
