fn main() {
    let input = std::fs::read_to_string("./inputs/day3.txt").unwrap();

    // PART 1 //

    let mut zeroes = [0; 12];
    let mut ones   = [0; 12];

    for line in input.lines() {
        for (index, c) in line.chars().enumerate() {
            if c == '0' {
                zeroes[index] += 1;
            } else {
                ones[index] += 1;
            }
        }
    }

    let mut most = String::new();
    let mut least = String::new();

    for i in 0..12 {
        if zeroes[i] > ones[i] {
            most.push('0');
            least.push('1');
        } else {
            most.push('1');
            least.push('0');
        }
    }

    let gamma = isize::from_str_radix(&most, 2).unwrap();
    let epsilon = isize::from_str_radix(&least, 2).unwrap();

    println!("Part 1: {}", gamma * epsilon);


    // PART 2 //

    let mut numbers1: Vec<&str> = input.lines().collect();
    let mut numbers2: Vec<&str> = input.lines().collect();

    for i in 0..12 {
        if numbers1.len() == 1 {
            break;
        }

        let (zeroes, ones) = numbers1.iter().fold((0, 0), |(zeroes, ones), number| {
            if number.chars().nth(i).unwrap() == '0' {
                (zeroes + 1, ones)
            } else {
                (zeroes, ones + 1)
            }
        });

        let common = if zeroes > ones {
            '0'
        } else {
            '1'
        };

        numbers1 = numbers1.into_iter().filter(|num| {
            num.chars().nth(i).unwrap() == common
        }).collect();
    }

    for i in 0..12 {
        if numbers2.len() == 1 {
            break;
        }

        let (zeroes, ones) = numbers2.iter().fold((0, 0), |(zeroes, ones), number| {
            if number.chars().nth(i).unwrap() == '0' {
                (zeroes + 1, ones)
            } else {
                (zeroes, ones + 1)
            }
        });

        let common = if zeroes > ones {
            '1'
        } else {
            '0'
        };

        numbers2 = numbers2.into_iter().filter(|num| {
            num.chars().nth(i).unwrap() == common
        }).collect();
    }

    let o2 = isize::from_str_radix(numbers1[0], 2).unwrap();
    let co2 = isize::from_str_radix(numbers2[0], 2).unwrap();

    println!("Part 2: {}", co2 * o2);
}