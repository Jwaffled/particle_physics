use crate::{
    components::{physics::*, render::*},
    physics::{calc_position, calc_velocity, Vec2},
};
use hecs::{PreparedQuery, World, Entity};
use sdl2::{rect::Rect, render::Canvas, video::Window};

pub fn physics_system(
    world: &mut World,
    query: &mut PreparedQuery<(
        &mut PositionComponent,
        &mut VelocityComponent,
        &mut AccelerationComponent,
        &MassComponent,
    )>,
    dt: &DeltaTime,
) {
    for (id, (pos, vel, acc, mass)) in query.query_mut(world) {
        vel.value = calc_velocity(&vel.value, &acc.value, dt);
        pos.value = calc_position(&vel.value, &pos.value, dt);
        // println!(
        //     "Entity {:?}: Vel: {:?}, Pos: {:?}",
        //     id, vel.value, pos.value
        // );
    }
}

pub fn render_system(
    world: &mut World,
    query: &mut PreparedQuery<(&RenderComponent, &PositionComponent)>,
    canvas: &mut Canvas<Window>,
) -> Result<(), String> {
    for (id, (render, pos)) in query.query_mut(world) {
        canvas.set_draw_color(render.color);
        canvas.draw_rect(Rect::new(pos.value.x as i32, pos.value.y as i32, 50, 50))?;
    }

    Ok(())
}

pub fn cleanup_system(
    world: &mut World,
    query: &mut PreparedQuery<(&PositionComponent)>,
    width: u32,
    height: u32
) {
    let mut to_remove = Vec::new();

    for (id, pos) in query.query(world).iter() {
        if(pos.value.y >= height as f32 * 10.0  || pos.value.y <= -(height as f32) * 10.0) {
            to_remove.push(id);
        }
    }

    for id in to_remove {
        world.despawn(id);
        println!("Entity {:?} despawned", id);
    }
}

// Sweep and prune WIP
// This does NOT work
// If anyone would like to fix this
// Please open a PR
pub fn collision_system(
    world: &mut World,
    query: &mut PreparedQuery<(
        &PositionComponent,
        &SizeComponent,
        &MassComponent,
        &CollisionComponent,
    )>,
    width: u32,
    height: u32
) {
    let mut intervals = Vec::new();
    for (id, (pos, size, ..)) in query.query(world).iter() {
        let start_x = pos.value.x as i32 - size.width as i32 / 2;
        let end_x = pos.value.x as i32 + size.width as i32 / 2;
        intervals.push((start_x, end_x, id));
    }

    intervals.sort_by(|a, b| a.0.cmp(&b.0));

    let mut potential_coll = Vec::new();
    if intervals.len() < 1 {
        return;
    }
    for i in 0..intervals.len() - 1 {
        if intervals[i + 1].0 <= intervals[i].1 {
            potential_coll.push((intervals[i].2, intervals[i + 1].2));
        }
    }

    for (entity1, entity2) in potential_coll {
        let pos1 = world.get::<PositionComponent>(entity1).unwrap();
        let pos2 = world.get::<PositionComponent>(entity2).unwrap();
        let size1 = world.get::<SizeComponent>(entity1).unwrap();
        let size2 = world.get::<SizeComponent>(entity2).unwrap();

        if check_collision((&pos1, &size1), (&pos2, &size2)) {
            let mut vel1 = world.get_mut::<VelocityComponent>(entity1).unwrap();
            let mut vel2 = world.get_mut::<VelocityComponent>(entity2).unwrap();
            let mass1 = world.get::<MassComponent>(entity1).unwrap();
            let mass2 = world.get::<MassComponent>(entity2).unwrap();
            let col1 = world.get::<CollisionComponent>(entity1).unwrap();
            let col2 = world.get::<CollisionComponent>(entity2).unwrap();
            // let (nvel1, nvel2) = resolve_collision((&vel1, &mass1), (&vel2, &mass2));
            // vel1.value = nvel1;
            // vel2.value = nvel2;
        }

    }
}

fn check_collision(entity1: (&PositionComponent, &SizeComponent), entity2: (&PositionComponent, &SizeComponent)) -> bool {
    let pos1 = entity1.0;
    let pos2 = entity2.0;
    let size1 = entity1.1;
    let size2 = entity2.1;

    let left1 = pos1.value.x as i32 - size1.width as i32 / 2;
    let right1 = pos1.value.x as i32 + size1.width as i32 / 2;
    let top1 = pos1.value.y as i32 - size1.height as i32 / 2;
    let bottom1 = pos1.value.y as i32 + size1.height as i32 / 2;

    let left2 = pos2.value.x as i32 - size2.width as i32 / 2;
    let right2 = pos2.value.x as i32 + size2.width as i32 / 2;
    let top2 = pos2.value.y as i32 - size2.height as i32 / 2;
    let bottom2 = pos2.value.y as i32 + size2.height as i32 / 2;

    left1 > right2 || right1 < left2 || top1 > bottom2 || bottom1 < top2
}