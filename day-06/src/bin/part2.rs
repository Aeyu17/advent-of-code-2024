fn main() {
    let data = include_str!("./input1.txt").to_string();
    let result = day_06(data);
    println!("Result: {}", result);
}

fn day_06(input: String) -> i32 {
    // this is horrible unoptimised I'm sorry
    let mut starting_map: Vec<Vec<char>> =  input.split('\n').map(|x| x.to_string().chars().collect()).collect();

    let starting_x = starting_map.iter().position(|x| x.contains(&'^')).unwrap();
    let starting_y = starting_map[starting_x].iter().position(|&x| x == '^').unwrap();

    println!("Initial position: {}, {}", starting_x, starting_y);

    let mut count = 0;
    
    // populate starting map with possible positions
    {
        let mut x = starting_x;
        let mut y = starting_y;
        let mut direction = Direction::UP;
        starting_map[x][y] = 'X';

        while !((direction == Direction::UP && x == 0) || (direction == Direction::DOWN && x == starting_map.len() - 1) || (direction == Direction::LEFT && y == 0) || (direction == Direction::RIGHT && y == starting_map[x].len() - 1)) {
            match direction {
                Direction::UP => {
                    if starting_map[x-1][y] == '#' {
                        direction = Direction::RIGHT;
                    } else {
                        x -= 1;
                    }
                },
                Direction::DOWN => {
                    if starting_map[x+1][y] == '#' {
                        direction = Direction::LEFT;
                    } else {
                        x += 1;
                    }
                },
                Direction::LEFT => {
                    if starting_map[x][y-1] == '#' {
                        direction = Direction::UP;
                    } else {
                        y -= 1;
                    }
                },
                Direction::RIGHT => {
                    if starting_map[x][y+1] == '#' {
                        direction = Direction::DOWN;
                    } else {
                        y += 1;
                    }
                },
            }
    
            starting_map[x][y] = 'X';
        }
    }

    for x_block in 0..starting_map.len() {
        for y_block in 0..starting_map[x_block].len() {
            if starting_map[x_block][y_block] != 'X' || (x_block == starting_x && y_block == starting_y) {
                continue;
            }

            let mut map = starting_map.clone();
            map[x_block][y_block] = '#';

            let mut direction = Direction::UP;
            let mut x = starting_x;
            let mut y = starting_y;

            let mut path: Vec<(usize, usize, Direction)> = Vec::new();

            let mut loop_detected = false;

            while !((direction == Direction::UP && x == 0) || (direction == Direction::DOWN && x == map.len() - 1) || (direction == Direction::LEFT && y == 0) || (direction == Direction::RIGHT && y == map[x].len() - 1)) {
                if path.contains(&(x, y, direction.clone())) {
                    loop_detected = true;
                    break;
                }

                path.push((x, y, direction));

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
            }

            if loop_detected {
                count += 1;
            }
        }
    }

    count
}

#[derive(PartialEq)]
#[derive(Clone, Copy)]
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
        assert_eq!(result, 6);
    }
}