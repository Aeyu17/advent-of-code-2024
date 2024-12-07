fn main() {
    let data = include_str!("./input1.txt").to_string();
    let result = day_07(data);
    println!("Result: {}", result);
}

fn day_07(input: String) -> i128 {
    let lines: Vec<String> = input.split('\n').map(|x| x.to_string()).collect();

    let mut answer = 0;

    for line in lines.iter() {
        let parsed_line: Vec<String> = line.split(':').map(|x| x.to_string()).collect();
        let result: i128 = parsed_line[0].parse().unwrap();
        let arguments: Vec<i128> = parsed_line[1].trim().split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();

        if check_possible(&result, &arguments, 1, arguments[0]) {
            answer += result;
        }
    }

    answer
}

fn concatenate(a: i128, b: i128) -> i128 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}

fn check_possible(result: &i128, arguments: &Vec<i128>, index: usize, starting_num: i128) -> bool {
    if starting_num == *result && index == arguments.len() {
        return true;
    }

    if index >= arguments.len() || starting_num > *result {
        return false;
    }

    return check_possible(result, arguments, index+1, starting_num + arguments[index]) || check_possible(result, arguments, index+1, starting_num * arguments[index])
        || check_possible(result, arguments, index+1, concatenate(starting_num, arguments[index]));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_07_test() {
        let data = include_str!("./test_input1.txt").to_string();
        let result = day_07(data);
        assert_eq!(result, 11387);
    }
}
