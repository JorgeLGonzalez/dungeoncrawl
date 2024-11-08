use super::helpers::Damager;
use crate::prelude::*;

#[system]
#[read_component(Carried)]
#[read_component(Damage)]
#[read_component(Player)]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    gather_victims(ecs)
        .iter()
        .for_each(|(message, attacker, victim)| {
            let mut damager = Damager::new(*attacker, *victim, ecs);
            damager.attack(ecs);

            if damager.should_terminate() {
                damager.terminate(commands);
            }

            commands.remove(*message);
        });
}

fn gather_victims(ecs: &SubWorld) -> Vec<(Entity, Entity, Entity)> {
    <(Entity, &WantsToAttack)>::query()
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect()
}
