use num_bigint::BigInt;

pub fn p025() {
    // BigInt rescues the day :)
    let mut pre = BigInt::from(1);
    let mut cur = BigInt::from(1);
    for i in 3.. {
        // calculate the ith fibonacci number
        cur = cur + pre.clone();
        pre = cur.clone() - pre;
        // convert the BigInt to String to get the amount of digits (length of the string)
        if cur.to_string().len() == 1000 {
            println!("{}", i);
            break;
        }
    }
}