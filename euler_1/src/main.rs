fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        //println!("{}", n);
        if n % 3 == 0 {
            println!("{} is a multiple of 3", n);
            continue;
        }
        if n % 5 == 0 {
            println!("{} is a multiple of 5", n);
            continue;
        }


    }
}
