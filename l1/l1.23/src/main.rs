fn main() {
    let p1 = Point::new(0, 4);
    let p2 = Point::new(3, 0);
    let dist = p1.distance(&p2);
    println!("Distance = {dist}");
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new<T>(x: T, y: T) -> Point where T: Into<f64> {
        Point { x: x.into(), y: y.into() }
    }

    fn distance(&self, other: &Point) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;

        // Чтобы пропустить лишнее возведение в квадрат и извлечение корня
        if x == 0.0 {
            return y.abs();
        }
        if y == 0.0 {
            return x.abs();
        }

        let d = x*x + y*y;
        return d.sqrt();
    }
}