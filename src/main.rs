use std::f64::consts::PI;

trait AreaCalculator  {
    fn calculate_area(&self) -> f64;
}

struct Circle{
    radius: f64,
}

impl AreaCalculator for Circle{
    fn calculate_area(&self)->f64{
        PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl AreaCalculator for Triangle{
    fn calculate_area(&self)->f64{
        self.base * self.height * 0.5
    }
}

struct Square {
    side: f64,
}

impl AreaCalculator for Square {
    fn calculate_area(&self) -> f64 {
        self.side * self.side
    }
}


fn calculate_area<T>(shape: T) where T: AreaCalculator {
    let area = shape.calculate_area();
    println!("The area is: {:.2}", area);
}


fn main(){
    let circle = Circle { radius: 3.0 };
    let triangle = Triangle { base: 4.0, height: 5.0 };
    let square = Square { side: 6.0 };

    calculate_area(circle);
    calculate_area(triangle);
    calculate_area(square);
}
