use std::collections::HashMap;

pub fn p026() {
    // 1/'max_d' with the longest recurring cycle so far
    let mut max_d = 0;
    // 'max_length' contains the length of the longest recurring cycle so far
    let mut max_length = 0;
    // iterating over every possible d
    for d in 2..1000 {
        let mut remainder = 1;
        // hashmap: keys are all found remainders and the value is the loop iteration in which it was found
        let mut remainders : HashMap<i32, i32> = HashMap::new();
        for j in 0..{
            // calculate the next remainder
            remainder = (remainder * 10) % d;
            // if the rmainder is 0 there is no recurring cycle
            if remainder == 0 {
                break;
            }
            // if a remainder appears multiple times, there is a recurring cycle
            if remainders.contains_key(&remainder) {
                // 'length' is the length of the recurring cycle
                let length = j - remainders.get(&remainder).unwrap();
                // if there is  a new max, update the max values
                if length > max_length {
                    max_length = length;
                    max_d = d;
                }
                break;
            }else {
                remainders.insert(remainder, j);
            }
        }
    }
    println!("{}", max_d);
}