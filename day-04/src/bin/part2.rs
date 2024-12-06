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
            if crossword[i][j] != 'A' {
                continue;
            }

            if contains_xmas(&crossword, i, j) {
                matches += 1;
            }
        }
    }

    matches
}

fn contains_xmas(crossword: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x < 1 || x > crossword.len() - 2 || y < 1 || y > crossword[x].len() - 2 {
        return false;
    }

    ((crossword[x-1][y-1] == 'M' && crossword[x+1][y+1] == 'S') || (crossword[x-1][y-1] == 'S' && crossword[x+1][y+1] == 'M')) &&
    ((crossword[x-1][y+1] == 'M' && crossword[x+1][y-1] == 'S') || (crossword[x-1][y+1] == 'S' && crossword[x+1][y-1] == 'M'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_04_test() {
        let data = include_str!("./test_input2.txt").to_string();
        let result = day_04(data);
        assert_eq!(result, 9);
    }
}