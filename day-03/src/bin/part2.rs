fn main() {
    let data = include_str!("./input1.txt").to_string();
    let result = day_03(data);
    println!("Result: {}", result);
}

fn day_03(input: String) -> i32 {
    let mut product = 0;
    let mut active = true;

    let mut remaining_input = input.clone();
    loop {
        // find mul
        let mul_location = remaining_input.find("mul(");
        let do_location = remaining_input.find("do()");
        let dont_location = remaining_input.find("don\'t()");

        if mul_location.is_none() {
            break;
        }

        if do_location.is_some() && dont_location.is_some() {
            let next_op = std::cmp::min(std::cmp::min(mul_location.unwrap(), do_location.unwrap()), dont_location.unwrap());

            if next_op == do_location.unwrap() {
                active = true;
                remaining_input = remaining_input[do_location.unwrap()+4..].to_string();
                continue;
            }

            if next_op == dont_location.unwrap() {
                active = false;
                remaining_input = remaining_input[dont_location.unwrap()+7..].to_string();
                continue;
            }
        } else if do_location.is_some() {
            let next_op = std::cmp::min(mul_location.unwrap(), do_location.unwrap());

            if next_op == do_location.unwrap() {
                active = true;
                remaining_input = remaining_input[do_location.unwrap()+4..].to_string();
                continue;
            }
        } else if dont_location.is_some() {
            let next_op = std::cmp::min(mul_location.unwrap(), do_location.unwrap());

            if next_op == dont_location.unwrap() {
                active = false;
                remaining_input = remaining_input[dont_location.unwrap()+7..].to_string();
                continue;
            }
        }
        // mul is the next thing
        if !active {
            remaining_input = remaining_input[mul_location.unwrap()+5..].to_string();
            continue;
        }

        remaining_input = remaining_input[mul_location.unwrap()+4..].to_string();

        // grab first number
        let (first_number, parse_output) = parse_number(&remaining_input);
        remaining_input = parse_output;

        // remove comma
        if remaining_input[..1].to_string() != "," {
            continue;
        }
        remaining_input = remaining_input[1..].to_string();

        // grab second number
        let (second_number, parse_output) = parse_number(&remaining_input);
        remaining_input = parse_output;

        // remove last parenthesis
        if remaining_input[..1].to_string() != ")" {
            continue;
        }
        remaining_input = remaining_input[1..].to_string();

        product += first_number * second_number;
    }
    product
}

fn parse_number(input: &String) -> (i32, String) {
    let mut number = 0;

    let mut remaining_input = input.clone();

    loop {
        let output = remaining_input[..1].parse::<i32>();

        match output {
            Ok(_) => number = number * 10 + output.unwrap(),
            Err(_) => break,
        }

        remaining_input = remaining_input[1..].to_string();
    }

    (number, remaining_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_03_test() {
        let data = include_str!("./test_input2.txt").to_string();
        let result = day_03(data);
        assert_eq!(result, 48);
    }
}