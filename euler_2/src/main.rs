use std::{thread, time};

fn main() {
    let mut f: [i64; 3] = [0, 1, 0];

    while f[1] <= 100 {
        f[2] = f[0] + f[1];
        f[0] = f[1];
        f[1] = f[2];
        //println!("array is {:?}",f);
        println!("{}", f[0]);
        thread::sleep(time::Duration::from_secs(1));

    }

    
}
