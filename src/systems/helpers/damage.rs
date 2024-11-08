use crate::prelude::*;

pub struct Damager {
    attacker: Entity,
    killed: bool,
    player_victim: bool,
    victim: Entity,
}

impl Damager {
    pub fn new(attacker: Entity, victim: Entity, ecs: &SubWorld) -> Self {
        Self {
            attacker,
            killed: false,
            player_victim: is_player(victim, ecs),
            victim,
        }
    }

    pub fn attack(&mut self, ecs: &mut SubWorld) {
        let damage = self.base_damage(ecs) + self.weapon_damage(ecs);

        let mut victim_entity = ecs.entry_mut(self.victim).unwrap();
        let health = victim_entity.get_component_mut::<Health>();

        self.killed = if let Ok(health) = health {
            println!("Health before attack: {}", health.current);
            health.current -= damage;
            println!("Health after attack: {}", health.current);

            health.current < 1
        } else {
            panic!("*** WARNING: attacked victim lacks Health!");
        }
    }

    pub fn should_terminate(&self) -> bool {
        self.killed && !self.player_victim
    }

    pub fn terminate(&self, commands: &mut CommandBuffer) {
        assert!(self.killed);
        assert!(!self.player_victim);

        println!("\tEnemy terminated!");

        commands.remove(self.victim);
    }

    fn base_damage(&self, ecs: &SubWorld) -> i32 {
        if let Ok(attack) = ecs.entry_ref(self.attacker) {
            if let Ok(damage) = attack.get_component::<Damage>() {
                damage.0
            } else {
                0
            }
        } else {
            0
        }
    }

    fn weapon_damage(&self, ecs: &SubWorld) -> i32 {
        <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == self.attacker)
            .map(|(_, damage)| damage.0)
            .sum()
    }
}

fn is_player(victim: Entity, ecs: &SubWorld) -> bool {
    ecs.entry_ref(victim)
        .unwrap()
        .get_component::<Player>()
        .is_ok()
}
