use std::iter::Iterator;

struct Fibonacci {
    a: u32,
    b: u32,
}

// impl Fibonacci {
//     fn new() ->  Self {
//         Fibonacci {
//             a: 1, 
//             b: 0
//         }
//     }
// }

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(& mut self) -> Option<Self::Item>{
        let t = self.b;
        self.b = self.a;
        self.a += t;
        Some(t)
    }
}

fn fibonacci_numbers() -> Fibonacci {
        Fibonacci { a: 1, b: 0 }
    }

fn main() {
    let fibo = fibonacci_numbers();
    
    let rs: Vec<u32> = fibo.take(10).collect();
    println!("{:?}", rs);
}