use std::f64::consts::PI;

trait Polygon {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn radius(&self) -> f64;
}

struct CanonicalPolygon {
    num_sides: i32,
    side_length: f64,
}

impl Polygon for CanonicalPolygon {
    fn area(&self) -> f64 {
        let apothem = self.side_length / (2.0 *
            (PI / self.num_sides as f64).tan());
        self.perimeter() * apothem / 2.0
    }
    fn perimeter(&self) -> f64 {
        self.num_sides as f64 * self.side_length
    }
    fn radius(&self) -> f64 {
        self.side_length / (2.0 * (PI / self.num_sides as f64).sin())
    }
}


fn main() {
    for side_length in [1.0, 10.0, 100.0] {
        for num_sides in [6, 12, 24, 128, 256, 512, 1024, 2048, 65536] {
            let polygon = CanonicalPolygon { num_sides, side_length };
            let radius = polygon.radius();
            let area = polygon.area();
            let circle_area = PI * radius * radius;
            println!(
                "Side Length: {}, Num Sides: {}, Radius: {}, Area: {}, Circle Area: {}",
                side_length, num_sides, radius, area, circle_area
            )
        }
    }
}
