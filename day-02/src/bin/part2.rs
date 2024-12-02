fn main() {
    let data = include_str!("./input1.txt").to_string();
    let result = day_02(data);
    println!("Result: {}", result);
}

fn day_02(input: String) -> i32 {
    let mut safe_lines = 0;

    for line in input.split("\n").map(|x| x.to_string()) {
        let data: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();

        if check_safety(&data) {
            safe_lines += 1;
            continue;
        }

        // goes here if not safe, check for removals

        for i in 0..data.len() {

            let mut cloned_data = data.clone();
            cloned_data.remove(i);

            if check_safety(&cloned_data) {
                safe_lines += 1;
                break;
            }
        }

    }

    safe_lines
}

fn check_safety(data: &Vec<i32>) -> bool {
    let increasing = data[1] > data[0];

    let mut safe = true;

    for i in 1..data.len() {
        let current = data[i];
        let previous = data[i-1];

        if !((increasing && current > previous && current - previous <= 3) || (!increasing && current < previous && previous - current <= 3)) {
            safe = false;
            break;
        }
    }

    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_02_test() {
        let data = include_str!("./test_input1.txt").to_string();
        let result = day_02(data);
        assert_eq!(result, 4);
    }
}