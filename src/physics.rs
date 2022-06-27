use std::ops::{Neg, Mul, Add, Sub, Div};

use crate::components::physics::{DeltaTime, VelocityComponent, MassComponent, CollisionComponent};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn scalar(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn sqrt(self) -> Vec2 {
        Vec2 {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[inline]
pub fn calc_position(velocity: &Vec2, position: &Vec2, dt: &DeltaTime) -> Vec2 {
    let dt = dt.0.as_secs_f32();
    Vec2::new(
        position.x as f32 + velocity.x * dt,
        position.y as f32 + velocity.y * dt,
    )
}

#[inline]
pub fn calc_velocity(velocity: &Vec2, a: &Vec2, dt: &DeltaTime) -> Vec2 {
    let dt = dt.0.as_secs_f32();
    Vec2::new(velocity.x + a.x * dt, velocity.y + a.y * dt)
}

#[inline]
pub fn calc_acceleration(force: &Vec2, mass: f32) -> Vec2 {
    Vec2::new(force.x / mass, force.y / mass)
}