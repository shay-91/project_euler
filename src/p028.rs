pub fn p028(){
    // 1001 x 1001 matrix consists of 501 squares. the center square is 1x1. followed by a 3x3 square and a 5x5 square. and so on...
    // sum of the center square is 1 (it just contains the 1)
    let mut sum = 1;
    let mut current_number = 1;
    // a loop iteration for every square
    for i in 1..=500 {
        // add every edge of the square to the sum
        for _j in 0..4 {
            current_number += 2 * i;
            sum += current_number;
        }
    }
    println!("{}", sum);
}