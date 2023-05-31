// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use std::ops::{Add, Deref};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn magnitude(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

    fn dist(&self, point_2: &Point) -> f64 {
        let x_dist_squared = (point_2.x - self.x).pow(2);
        let y_dist_squared = (point_2.y - self.y).pow(2);

        ((x_dist_squared + y_dist_squared) as f64).sqrt()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Deref for Polygon {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

impl Polygon {
    fn new() -> Self {
        Self { points: Vec::new() }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point)
    }

    fn left_most_point(&self) -> Option<Point> {
        let mut lmp = Some(self.points[0]);

        for p in self.points.iter() {
            if lmp.unwrap().x > p.x {
                lmp = Some(*p);
            }
        }

        lmp
    }

    fn perimeter(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        let mut perimeter = 0.0;
        let mut last_point = self.points[0];
        for point in &self.points[1..] {
            perimeter += last_point.dist(point);
            last_point = *point;
        }
        perimeter += last_point.dist(&self.points[0]);

        perimeter
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    fn new(point: Point, radius: i32) -> Self {
        Self {
            center: point,
            radius,
        }
    }

    fn circumference(&self) -> f64 {
        2.0 * self.radius as f64 * std::f64::consts::PI
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Self::Polygon(poly) => poly.perimeter(),
            Self::Circle(c) => c.circumference(),
        }
    }
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Self::Polygon(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Self::Circle(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(&p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

pub fn run() {
    let mut poly = Polygon::new();
    poly.add_point(Point::new(12, 13));
    poly.add_point(Point::new(17, 11));

    println!("{:?}", poly.perimeter());
    poly.add_point(Point::new(16, 16));

    println!("{:?}", poly.perimeter());
}
