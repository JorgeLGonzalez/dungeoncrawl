use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    gather_victims(ecs).iter().for_each(|(message, victim)| {
        damage(ecs, *victim)
            .filter(|health| *health < 1)
            .inspect(|_| {
                println!("\tVictim Died :-(");
                commands.remove(*victim);
            });

        commands.remove(*message);
    });
}

fn gather_victims(ecs: &SubWorld) -> Vec<(Entity, Entity)> {
    <(Entity, &WantsToAttack)>::query()
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect()
}

fn damage(ecs: &mut SubWorld, victim: Entity) -> Option<i32> {
    if let Ok(health) = ecs.entry_mut(victim).unwrap().get_component_mut::<Health>() {
        println!("Health before attack: {}", health.current);
        health.current -= 1;
        println!("Health after attack: {}", health.current);

        Some(health.current)
    } else {
        None
    }
}
