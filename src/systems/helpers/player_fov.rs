use crate::prelude::*;

pub fn player_fov<'a>(ecs: &'a SubWorld<'a>) -> &'a FieldOfView {
    <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap()
}
