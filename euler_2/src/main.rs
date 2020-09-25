//use std::{thread, time};

fn main() {
    let max = 4000000;
    let mut sum = 0;
    let mut f: [i64; 3] = [0, 1, 0];

    while f[1] < max {
        f[2] = f[0] + f[1];
        f[0] = f[1];
        f[1] = f[2];


        if f[1] % 2 == 0 {
            sum += f[1];
            println!("{} is even. Sum: {}", f[1], sum);
            continue;
        }
        

    }
    println!("Sum: {}", sum)

    
}
