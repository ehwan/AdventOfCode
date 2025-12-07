use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let ops = str::from_utf8(&lines.pop().unwrap())
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    println!("{:?}", ops);

    let mut x = lines.iter().map(|s| s.len() - 1).max().unwrap();
    let mut answer = 0;
    for op in ops.into_iter().rev() {
        let mut numbers = Vec::new();
        loop {
            let mut found_digit = false;
            let mut num = 0;
            for row in lines.iter() {
                let digit = if x < row.len() { row[x] } else { b' ' };
                if digit != b' ' {
                    found_digit = true;
                    num = num * 10 + (digit - b'0') as u64;
                }
            }
            println!("{}", num);
            if found_digit {
                numbers.push(num);
            }
            if x == 0 {
                break;
            }
            x -= 1;
            if found_digit == false {
                break;
            }
        }
        println!("{:?}", numbers);

        match op.as_str() {
            "+" => {
                let mut sum = 0;
                for num in numbers {
                    sum += num;
                }
                answer += sum;
            }
            "*" => {
                let mut prod = 1;
                for num in numbers {
                    prod *= num;
                }
                answer += prod;
            }
            _ => panic!("Invalid Operation"),
        }
    }
    println!("{}", answer);
}
