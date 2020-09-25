//use std::{thread, time};

fn main() {
    let mut sum = 0;
    let mut f: [i64; 3] = [0, 1, 0];

    while f[1] <= 100 {
        f[2] = f[0] + f[1];
        f[0] = f[1];
        f[1] = f[2];
        //println!("array is {:?}",f);
        
        //thread::sleep(time::Duration::from_secs(1));

        if f[1] % 2 == 0 {
            sum += f[1];
            println!("{} is even. Sum: {}", f[1], sum);
            continue;
        }
        println!("{}", f[0]);

    }

    
}
