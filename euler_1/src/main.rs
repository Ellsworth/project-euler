fn main() {
    // 'sum' will be the running sum of the multiples.
    let mut sum = 0;

    for n in 1..1000 {

        if n % 3 == 0 {
            sum += n;
            println!("{} is a multiple of 3. Sum: {}", n, sum);
            continue;
        }
        
        if n % 5 == 0 {
            sum += n;
            println!("{} is a multiple of 5. Sum: {}", n, sum);
            continue;
        }


    }
    println!("Final sum: {}", sum)
}
