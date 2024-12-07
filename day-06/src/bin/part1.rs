fn main() {
    let data = include_str!("./input1.txt").to_string();
    let result = day_06(data);
    println!("Result: {}", result);
}

fn day_06(input: String) -> i32 {
    let mut map: Vec<Vec<char>> = input.split('\n').map(|x| x.to_string().chars().collect()).collect();

    let mut x = map.iter().position(|x| x.contains(&'^')).unwrap();
    let mut y = map[x].iter().position(|&x| x == '^').unwrap();
    println!("Initial position: {}, {}", x, y);

    map[x][y] = 'X';

    let mut direction = Direction::UP;

    while !((direction == Direction::UP && x == 0) || (direction == Direction::DOWN && x == map.len() - 1) || (direction == Direction::LEFT && y == 0) || (direction == Direction::RIGHT && y == map[x].len() - 1)) {
        match direction {
            Direction::UP => {
                if map[x-1][y] == '#' {
                    direction = Direction::RIGHT;
                } else {
                    x -= 1;
                }
            },
            Direction::DOWN => {
                if map[x+1][y] == '#' {
                    direction = Direction::LEFT;
                } else {
                    x += 1;
                }
            },
            Direction::LEFT => {
                if map[x][y-1] == '#' {
                    direction = Direction::UP;
                } else {
                    y -= 1;
                }
            },
            Direction::RIGHT => {
                if map[x][y+1] == '#' {
                    direction = Direction::DOWN;
                } else {
                    y += 1;
                }
            },
        }

        map[x][y] = 'X';
    }

    let mut count = 0;
    for row in map {
        for point in row {
            if point == 'X' {
                count += 1;
            }
        }
    }

    count
}

#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_06_test() {
        let data = include_str!("./test_input1.txt").to_string();
        let result = day_06(data);
        assert_eq!(result, 41);
    }
}