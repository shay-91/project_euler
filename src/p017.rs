pub fn p017() {
    // of course, there were no mistakes ;)
    // |one thousand| = 11;
    let mut sum = 11;
    for i in 1..1000 {
        let chars: Vec<char> = i.to_string().chars().collect();
        // 'position' is the position of the current digit (from left to right)
        let mut position = 0;
        // three characters => 3 digits (something with "hundred")
        if chars.len() == 3 {
            // |hundred| = 7
            sum += 7;
            if chars[1] != '0' || chars[2] != '0' {
                // |and| = 3
                sum += 3;
            }
            // example: 2 from two hundred...
            sum += get_length_first_digit(chars[0]);
            position += 1;
        }
        // at least 2 characters => at least two digits
        if chars.len() > 1 {
            if chars[position] == '1' {
                // a number between 10 and 19
                sum += teens(chars[position + 1]);
                continue;
            }
            // a number between 2x and 9x (not counting the x in this step)
            sum += get_length_second_digit(chars[position]);
            position += 1;
        }
        sum += get_length_first_digit(chars[position]);
    }
    println!("{}", sum);
}

fn get_length_first_digit(c: char) -> u32 {
    match c {
        // |one| = |two| = |six| = 3
        '1' | '2' | '6' => 3,
        // |four| = |five| = |nine| = 4
        '4' | '5' | '9' => 4,
        // |three| = |seven| = |eight| = 5
        '3' | '7' | '8' => 5,
        // || = 0 (zero)
        _ => 0,
    }
}

fn get_length_second_digit(c: char) -> u32 {
    match c {
        // |forty| = |fifty| = |sixty| = 5
        '4' | '5' | '6' => 5,
        // |twenty| = |thirty| = |eighty| = |ninety| = 6
        '2' | '3' | '8' | '9' => 6,
        // |seventy| = 7
        '7' => 7,
        _ => 0,
    }
}

fn teens(c: char) -> u32 {
    match c {
        // |ten| = 3
        '0' => 3,
        // |eleven| = |twelve| = 6
        '1' | '2' => 6,
        // |fivteen| = |sixteen| = 7
        '5' | '6' => 7,
        // |thirteen| = |fourteen| = |eighteen| = |nineteen| = 8
        '3' | '4' | '8' | '9' => 8,
        // |seventeen| = 9
        '7'  => 9,
        _ => 0,
    }
}
