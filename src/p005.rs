pub fn p005() {
    // get an iterator over all multiples of 20 in increasing order
    let mut multiples = (20..).step_by(20);
    // find the first multiple of 20 that is evenly divisible by all numbers from 2 to 19
    println!(
        "{}",
        multiples
            .find(|x| (2..20).all(move |y| x % y == 0))
            .unwrap()
    );
}
