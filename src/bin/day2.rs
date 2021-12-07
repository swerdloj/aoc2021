fn main() {
    let input = aoc2021::read_input(2);

    // PART 1 //

    let result = input.lines()
        .fold((0, 0), |(mut horizontal, mut depth), line| {
            let split = line.split_whitespace().collect::<Vec<_>>();
            let command = split[0];
            let value = split[1].parse::<u32>().unwrap();

            match command {
                "forward" => horizontal += value,
                "down" => depth += value,
                "up" => depth -= value,
                _ => unreachable!()
            }

            (horizontal, depth)
        });

    println!("Part 1: {}", result.0 * result.1);


    // PART 2 //

    let result = input.lines()
        .fold((0, 0, 0), |(mut horizontal, mut depth, mut aim), line| {
            let split = line.split_whitespace().collect::<Vec<_>>();
            let command = split[0];
            let value = split[1].parse::<u32>().unwrap();

            match command {
                "forward" => {
                    horizontal += value;
                    depth += aim * value;
                },
                "down" => aim += value,
                "up" => aim -= value,
                _ => unreachable!()
            }

            (horizontal, depth, aim)
        });

    println!("Part 1: {}", result.0 * result.1);
}