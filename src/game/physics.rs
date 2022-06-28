use iced::Point;

use super::Coord;

pub fn distance_between(a: &Point, b: &Point) -> f32 {
    ((a.x - b.x).powf(2.) + (a.y - b.y).powf(2.)).sqrt()
}

pub fn coord_to_midpoint(coord: &Coord, block: f32) -> Point {
    Point { x: coord.x as f32 * block + block / 2., y: coord.y as f32 * block + block / 2. }
}

pub fn point_to_coord(point: &Point, block: f32) -> Coord {
    Coord { x: (point.x / block).floor() as u8, y: (point.y / block).floor() as u8 }
}

pub fn top_left_point<N>(x: N, y: N, block: f32) -> Point
    where N: Into<f32> {
    Point::new(x.into() * block, y.into() * block)
}