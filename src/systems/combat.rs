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
            let is_player = is_player(ecs, *victim);

            damage(ecs, *attacker, *victim)
                .filter(|health| *health < 1 && !is_player)
                .inspect(|_| {
                    println!("\tVictim Died :-(");
                    commands.remove(*victim);
                });

            commands.remove(*message);
        });
}

fn damage(ecs: &mut SubWorld, attacker: Entity, victim: Entity) -> Option<i32> {
    let base_damage = if let Ok(v) = ecs.entry_ref(attacker) {
        if let Ok(damage) = v.get_component::<Damage>() {
            damage.0
        } else {
            0
        }
    } else {
        0
    };

    let weapon_damage: i32 = <(&Carried, &Damage)>::query()
        .iter(ecs)
        .filter(|(carried, _)| carried.0 == attacker)
        .map(|(_, damage)| damage.0)
        .sum();

    let final_damage = base_damage + weapon_damage;

    if let Ok(health) = ecs.entry_mut(victim).unwrap().get_component_mut::<Health>() {
        println!("Health before attack: {}", health.current);
        health.current -= final_damage;
        println!("Health after attack: {}", health.current);

        Some(health.current)
    } else {
        None
    }
}

fn gather_victims(ecs: &SubWorld) -> Vec<(Entity, Entity, Entity)> {
    <(Entity, &WantsToAttack)>::query()
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect()
}

fn is_player(ecs: &SubWorld, victim: Entity) -> bool {
    ecs.entry_ref(victim)
        .unwrap()
        .get_component::<Player>()
        .is_ok()
}
