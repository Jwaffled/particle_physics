use std::time::Duration;

use crate::physics::Vec2;

pub struct VelocityComponent {
    pub value: Vec2,
}

impl VelocityComponent {
    pub fn new(x: f32, y: f32) -> Self {
        VelocityComponent {
            value: Vec2::new(x, y),
        }
    }
}

pub struct PositionComponent {
    pub value: Vec2,
}

impl PositionComponent {
    pub fn new(x: f32, y: f32) -> Self {
        PositionComponent {
            value: Vec2::new(x, y),
        }
    }
}

pub struct MassComponent {
    pub value: f32,
}

impl MassComponent {
    pub fn new(mass: f32) -> Self {
        MassComponent { value: mass }
    }
}

pub struct AccelerationComponent {
    pub value: Vec2,
}

impl AccelerationComponent {
    pub fn new(x: f32, y: f32) -> Self {
        AccelerationComponent {
            value: Vec2::new(x, y),
        }
    }
}

pub struct DeltaTime(pub Duration);

impl Default for DeltaTime {
    fn default() -> Self {
        DeltaTime(Duration::from_millis(1))
    }
}

pub enum CollisionType {
    Elastic,
    Inelastic,
    PerfectlyInelastic,
}

pub struct CollisionComponent {
    pub col_type: CollisionType,
}

impl CollisionComponent {
    pub fn new(col_type: CollisionType) -> Self {
        CollisionComponent { col_type }
    }
}

pub struct SizeComponent {
    pub width: u32,
    pub height: u32,
}

impl SizeComponent {
    pub fn new(width: u32, height: u32) -> Self {
        SizeComponent {
            width,
            height,
        }
    }
}