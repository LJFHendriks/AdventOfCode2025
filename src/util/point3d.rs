use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

pub trait PointTrait: Add<Output = Self> + Sub<Output = Self> + Into<f64> + Copy {

}
impl<T: Add<Output = T> + Sub<Output = T> + Into<f64> + Copy> PointTrait for T {

}

impl<T: PointTrait> Point3D<T>
{
    pub fn new(x: T, y: T, z: T) -> Point3D<T> {
        Point3D { x, y, z }
    }

    pub fn distance(&self, other: &Point3D<T>) -> f64 {
        let dx = self.x.into() - other.x.into();
        let dy = self.y.into() - other.y.into();
        let dz = self.z.into() - other.z.into();
        (dx*dx + dy*dy + dz*dz).sqrt()
    }

    pub fn distance_from_origin(&self) -> f64 {
        let x = self.x.into();
        let y = self.y.into();
        let z = self.z.into();
        (x*x + y*y + z*z).sqrt()
    }
}

impl<T: PointTrait> Add for Point3D<T> {
    type Output = Point3D<T>;

    fn add(self, rhs: Self) -> Self {
        Point3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: PointTrait> Sub for Point3D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Point3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}