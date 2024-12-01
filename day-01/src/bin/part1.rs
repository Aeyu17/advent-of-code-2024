fn main() {

    let data = include_str!("./input1.txt").split("\n").map(|x| x.to_string());

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in data {
        let read_locations: Vec<i32> = line.split(' ').filter(|x| x != &"").map(|x| x.parse().unwrap()).collect();

        left.push(read_locations[0]);
        right.push(read_locations[1]);
    }

    left.sort();
    right.sort();

    assert_eq!(left.len(), right.len());

    let mut total_distance = 0;

    for i in 0..left.len() {
        let left_item = left[i];
        let right_item = right[i];

        total_distance += (left_item - right_item).abs();
    }

    println!("Total distance: {}", total_distance);
}