fn main() {
    let data = include_str!("./input1.txt").to_string();
    let result = day_05(data);
    println!("Result: {}", result);
}

fn day_05(input: String) -> i32 {
    let split_input: Vec<String> = input.split("\n\n").map(|x| x.to_string()).collect();
    let raw_rules: Vec<String> = split_input[0].split('\n').map(|x| x.to_string()).collect();
    let updates: Vec<String> = split_input[1].split('\n').map(|x| x.to_string()).collect();

    let mut rules: Vec<(i32, i32)> = Vec::new();

    for rule in raw_rules {
        let split_rule: Vec<i32> = rule.split('|').map(|x| x.parse().unwrap()).collect();
        rules.push((split_rule[0], split_rule[1]));
    }

    let mut answer = 0;

    for raw_update in updates {
        let update: Vec<i32> = raw_update.split(',').map(|x| x.parse().unwrap()).collect();

        let mut valid_update: bool = true;

        for rule in &rules {
            if update.iter().position(|&x| x == rule.0).is_none() || update.iter().position(|&x| x == rule.1).is_none() {
                continue;
            }

            let first_pos = update.iter().position(|&x| x == rule.0).unwrap() as i32;
            let second_pos = update.iter().position(|&x| x == rule.1).unwrap() as i32;

            if second_pos < first_pos {
                valid_update = false;
                break;
            }
        }

        if valid_update {
            answer += update[update.len()/2];
        }
        
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_05_test() {
        let data = include_str!("./test_input1.txt").to_string();
        let result = day_05(data);
        assert_eq!(result, 143);
    }
}