fn main() {

    let data = include_str!("./test_input1.txt").split("\n").map(|x| x.to_string());

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in data {
        let read_locations: Vec<i32> = line.split(' ').filter(|x| x != &"").map(|x| x.parse().unwrap()).collect();

        left.push(read_locations[0]);
        right.push(read_locations[1]);
    }

    assert_eq!(left.len(), right.len());

    let mut similarity = 0;

    for i in left {
        let count = right.iter().filter(|&x| *x == i).count() as i32;

        similarity += i * count;

    }

    println!("Similarity Score: {}", similarity);
}