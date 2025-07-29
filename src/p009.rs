pub fn p009() {
    // a has to be smaller than 333 because a < b < c and a + b + c = 1000
    for a in 1..333 {
        for b in a + 1.. {
            // there is an upper bound for b because b < c and a + b + c = 1000
            if b >= 1000 - a - b {
                break;
            }
            let c = 1000 - a - b;
            // check pythagorean triplet property
            if a * a + b * b == c * c {
                println!("{}", a * b * c);
                break;
            }
        }
    }
}
