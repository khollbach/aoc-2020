use std::hash::Hash;
use std::ops::Add;

pub trait Point: Clone + Copy + PartialEq + Eq + Hash {
    fn new(x: i32, y: i32) -> Self;

    fn adj_points(self) -> Vec<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point for Point3 {
    fn new(x: i32, y: i32) -> Self {
        Point3 { x, y, z: 0 }
    }

    fn adj_points(self) -> Vec<Point3> {
        let mut points = vec![];
        for &x in &[-1, 0, 1] {
            for &y in &[-1, 0, 1] {
                for &z in &[-1, 0, 1] {
                    let p = Point3 { x, y, z };
                    if p != Point3::default() {
                        points.push(self + p);
                    }
                }
            }
        }
        points
    }
}

impl Add<Point3> for Point3 {
    type Output = Point3;

    fn add(self, Point3 { x, y, z }: Point3) -> Point3 {
        let mut p = self;
        p.x += x;
        p.y += y;
        p.z += z;
        p
    }
}

impl Point for Point4 {
    fn new(x: i32, y: i32) -> Self {
        Point4 { x, y, z: 0, w: 0 }
    }

    fn adj_points(self) -> Vec<Point4> {
        let mut points = vec![];
        for &x in &[-1, 0, 1] {
            for &y in &[-1, 0, 1] {
                for &z in &[-1, 0, 1] {
                    for &w in &[-1, 0, 1] {
                        let p = Point4 { x, y, z, w };
                        if p != Point4::default() {
                            points.push(self + p);
                        }
                    }
                }
            }
        }
        points
    }
}

impl Add<Point4> for Point4 {
    type Output = Point4;

    fn add(self, Point4 { x, y, z, w }: Point4) -> Point4 {
        let mut p = self;
        p.x += x;
        p.y += y;
        p.z += z;
        p.w += w;
        p
    }
}
