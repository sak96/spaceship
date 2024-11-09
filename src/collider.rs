use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Collider {
    radius: f32,
    pub entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            entities: vec![],
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

fn collision(
    transform_1: &GlobalTransform,
    collider_1: &Collider,
    transform_2: &GlobalTransform,
    collider_2: &Collider,
) -> bool {
    let distance_between_center = transform_1
        .translation()
        .distance(transform_2.translation());
    let min_distance = collider_1.radius + collider_2.radius;
    distance_between_center <= min_distance
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut collision_map: Vec<Vec<Entity>> = Vec::new();
    for (entity_1, transform_1, collider_1) in query.iter() {
        let colliding_entities = query
            .iter()
            .filter_map(|(entity_2, transform_2, collider_2)| {
                ((entity_1 != entity_2)
                    & collision(transform_1, collider_1, transform_2, collider_2))
                .then_some(entity_2)
            })
            .collect();
        collision_map.push(colliding_entities)
    }

    for ((_, _, mut collider), colliding_entities) in query.iter_mut().zip(collision_map) {
        collider.entities = colliding_entities;
    }
}
