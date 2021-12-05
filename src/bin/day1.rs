fn main() {
    let input = std::fs::read_to_string("./inputs/day1.txt").unwrap();

    // PART 1 //

    let count = input.lines()
        // Ignore the initial comparison because there is nothing to compare it against (and -1 will always be the lower)
        .fold((-1, -1), |(mut increased, last), current_str| {
            let current = current_str.parse::<i32>().unwrap();

            if current > last {
                increased += 1;
            }

            (increased, current)
        }).0;

    println!("Part 1: {}", count);


    // PART 2 //
    
    let count = input.lines()
        // Ignore the initial 3 comparisons because the first window isn't formed until 3 numbers are seen
        .fold((-3, [-1, -1, -1], 0), |(mut increased, mut window, index), current_str| {
            let current = current_str.parse::<i32>().unwrap();

            let last_sum: i32 = window.iter().sum();
            window[index % 3] = current;
            let current_sum: i32 = window.iter().sum();

            if current_sum > last_sum {
                increased += 1;
            }
            
            (increased, window, index + 1)
        }).0;

    println!("Part 2: {}", count);
}