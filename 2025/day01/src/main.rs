use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut cur: i64 = 50;
    let mut through = 0;
    let mut end = 0;
    for line in input.lines() {
        let dir = match line.as_bytes()[0] {
            b'L' => -1,
            b'R' => 1,
            _ => {
                panic!("Invalid direction");
            }
        };

        let distance: i64 = line[1..].parse().unwrap();
        if distance == 0 {
            continue;
        }
        let next = cur + dir * distance;
        for i in cur.min(next)..=cur.max(next) {
            if i % 100 == 0 {
                through += 1;
            }
        }
        if cur == 0 {
            through -= 1;
        }
        if next % 100 == 0 {
            through -= 1;
            end += 1;
        }
        cur = next % 100;
    }
    println!("{}, {}, {}", through, end, through + end);
}
