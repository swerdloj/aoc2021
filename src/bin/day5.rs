fn main() {
    println!("Part 1: {}", solve(false));
    println!("Part 2: {}", solve(true));
}

fn solve(diagonals: bool) -> usize {
    let input = std::fs::read_to_string("./inputs/day5.txt").unwrap();

    let mut crossings = std::collections::HashMap::new();

    for line in input.lines() {
        let mut split = line.split(" -> ");

        let parse_point = |point: (i32, i32), value: &str| {
            if point.0 == -1 {
                (value.parse::<i32>().unwrap(), point.1)
            } else {
                (point.0, value.parse::<i32>().unwrap())
            }
        };

        let point1 = split.next().unwrap().split(',')
            .fold((-1, -1), parse_point);
        let point2 = split.next().unwrap().split(',')
            .fold((-1, -1), parse_point);

        if point1.0 == point2.0 { // Same xs
            for y in point1.1.min(point2.1) ..= point1.1.max(point2.1) {
                if let Some(count) = crossings.get_mut(&(point1.0, y)) {
                    *count += 1;
                } else {
                    crossings.insert((point1.0, y), 1);
                }
            }
        } else if point1.1 == point2.1 { // Same ys
            for x in point1.0.min(point2.0) ..= point1.0.max(point2.0) {
                if let Some(count) = crossings.get_mut(&(x, point1.1)) {
                    *count += 1;
                } else {
                    crossings.insert((x, point1.1), 1);
                }
            }
        } else if diagonals { // PART 2
            let slope = (point1.0 - point2.0) as f32 / (point1.1 - point2.1) as f32;

            if slope.abs() == 1.0 { // Diagonal
                let (direction, mut y) = if point1.0.min(point2.0) == point1.0 {
                    if point1.1 > point2.1 { // starting at point1 and it is higher than point2
                        (-1, point1.1)
                    } else { // starting at point1 and it is lower
                        (1, point1.1)
                    }
                } else if point2.1 > point1.1 { // starting at point2 and it is higher
                    (-1, point2.1)
                } else { // starting at point2 and it is lower
                    (1, point2.1)
                };

                for x in point1.0.min(point2.0) ..= point1.0.max(point2.0) {
                    if let Some(count) = crossings.get_mut(&(x, y)) {
                        *count += 1;
                    } else {
                        crossings.insert((x, y), 1);
                    }

                    y += direction;
                }
            }
        }
    }

    crossings.values().filter(|&x| *x >= 2).count()
}