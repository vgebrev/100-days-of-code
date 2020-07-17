fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &current in list {
        if current > largest {
            largest = current;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let nums = vec![34, 50, 25, 100, 65];
    let result = find_largest(&nums);
    println!("Largest from {:?} is {}", nums, result);

    let nums = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = find_largest(&nums);
    println!("Largest from {:?} is {}", nums, result);

    let chars = vec!['a', 'z', 'w', 'b', 'f', 'j', 'p', '0'];
    let result = find_largest(&chars);
    println!("Largest from {:?} is {}", chars, result);

    let p = Point { x: 10.0, y: 20.0 };
    println!("({}, 20) length = {}", p.x(), p.distance());
}
