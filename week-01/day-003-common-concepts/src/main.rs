fn main() {
    let mut n: u16 = 0;
    let mut fibbs: [u16; 10] = [0;10];
    while n < 10 {
        fibbs[n as usize] = fibonacci(n);
        n += 1;
    }

    for fibb in fibbs.iter() {
        println!("{}", fibb);
    }
    
}

fn fibonacci(n: u16) -> u16 {
    if n < 2 { 1 } else { fibonacci(n - 1) + fibonacci(n - 2) }
}