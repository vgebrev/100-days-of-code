#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width >= rectangle.width && self.height >= rectangle.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect = Rectangle {
        width: width1,
        height: height1
    };

    let rect2 = Rectangle {
        width: 40,
        height: 60
    };

    let rect3 = Rectangle::square(80);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    println!("rect2 is {:#?}", rect2);

    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("rect2 can hold rect: {}", rect2.can_hold(&rect));
    println!("rect3 can hold rect: {}", rect3.can_hold(&rect));
}

fn _area(width: u32, height: u32) -> u32 {
    width * height
}

fn _area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn _area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}