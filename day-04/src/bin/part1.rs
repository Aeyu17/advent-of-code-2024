use std::slice::Iter;

fn main() {
    let data = include_str!("./input1.txt").to_string();
    let result = day_04(data);
    println!("Result: {}", result);
}

fn day_04(input: String) -> i32 {
    let mut matches = 0;
    let crossword: Vec<Vec<char>> = input.split('\n').map(|x| x.to_string().chars().collect()).collect();

    for i in 0..crossword.len() {

        for j in 0..crossword[i].len() {
            if crossword[i][j] != 'X' {
                continue;
            }

            for direction in Directions::iterator() {
                if contains_xmas(&crossword, direction, i, j) {
                    matches += 1;
                }
            }
        }
    }

    matches
}

#[derive(Debug)]
enum Directions {
    NORTH,
    NORTHEAST,
    EAST,
    SOUTHEAST,
    SOUTH,
    SOUTHWEST,
    WEST,
    NORTHWEST,
}

impl Directions {
    pub fn iterator() -> Iter<'static, Directions> {
        static DIRECTIONS: [Directions; 8] = [Directions::NORTH, Directions::NORTHEAST, Directions::EAST, Directions::SOUTHEAST, Directions::SOUTH, Directions::SOUTHWEST, Directions::WEST, Directions::NORTHWEST];
        DIRECTIONS.iter()
    }
}

fn contains_xmas(crossword: &Vec<Vec<char>>, direction: &Directions, x: usize, y: usize) -> bool {
    let mut pattern: String = crossword[x][y].to_string();

    for i in 1..4 {
        match direction {
            Directions::NORTH => {
                if x < 3 {
                    return false;
                }
                pattern.push(crossword[x-i][y]);
            },
            Directions::NORTHEAST => {
                if x < 3 || y > crossword[x].len() - 4 {
                    return false;
                }
                pattern.push(crossword[x-i][y+i]);
            },
            Directions::EAST => {
                if y > crossword[x].len() - 4 {
                    return false;
                }
                pattern.push(crossword[x][y+i]);
            },
            Directions::SOUTHEAST => {
                if x > crossword.len() - 4 || y > crossword[x].len() - 4 {
                    return false;
                }
                pattern.push(crossword[x+i][y+i]);
            },
            Directions::SOUTH => {
                if x > crossword.len() - 4 {
                    return false;
                }
                pattern.push(crossword[x+i][y]);
            },
            Directions::SOUTHWEST => {
                if x > crossword.len() - 4 || y < 3 {
                    return false;
                }
                pattern.push(crossword[x+i][y-i]);
            },
            Directions::WEST => {
                if y < 3 {
                    return false;
                }
                pattern.push(crossword[x][y-i]);
            },
            Directions::NORTHWEST => {
                if x < 3 || y < 3 {
                    return false;
                }
                pattern.push(crossword[x-i][y-i]);
            },
        }
    }
    
    pattern == "XMAS"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_04_test() {
        let data = include_str!("./test_input1.txt").to_string();
        let result = day_04(data);
        assert_eq!(result, 18);
    }
}