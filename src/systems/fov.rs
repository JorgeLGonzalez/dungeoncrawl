use super::FieldOfView;
use crate::prelude::*;

/// Field of view system. See pp 349-352
pub fn fov(mut query: Query<(&PointC, &mut FieldOfView)>, map: Res<Map>) {
    query
        .iter_mut()
        .filter(|(_, fov)| fov.is_dirty)
        .for_each(|(pos, mut fov)| {
            fov.visible_tiles = field_of_view_set(pos.0, fov.radius, map.as_ref());
            fov.is_dirty = false;
        });
}
