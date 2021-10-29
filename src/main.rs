
struct Circle{
    radius: u32,
}

struct Triangle{
    base: u32,
    height: u32,
}

struct Square{
    edge: u32,
}

trait Graph{
    fn area(&self) -> u32;
}

impl Graph for Circle{
    fn area(&self) -> u32{
        println!("Circle {}", (self.radius as f32 * self.radius as f32 * 3.14) as u32);
        (self.radius as f32 * self.radius as f32 * 3.14) as u32
    }
}


impl Graph for Triangle{
    fn area(&self) -> u32{
        println!("Triangle {}", self.base * self.height / 2);
        self.base * self.height / 2
    }
}

impl Graph for Square{
    fn area(&self) -> u32{
        println!("Square {}", self.edge * self.edge);
        self.edge * self.edge
    }
}

fn area<T:Graph>(obj: T){
    obj.area();
}

fn main() {
    let circle: Circle = Circle{radius: 50};
    let square: Square = Square{edge: 50};
    let triangle: Triangle = Triangle{base: 50, height:100};
    area(circle);
    area(triangle);
    area(square);
    // println!("Hello, world!");
}
