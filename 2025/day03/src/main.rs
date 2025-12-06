use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    fn max_index(bytes: &[u8]) -> usize {
        bytes.len()
            - bytes
                .iter()
                .rev()
                .enumerate()
                .map(|(i, x)| (*x, i))
                .max()
                .unwrap()
                .1
            - 1
    }

    let mut answer = 0;
    const NUM: usize = 12;
    for line in input.trim().lines() {
        let line = line.as_bytes();
        let mut search_range = 0..line.len();

        let mut indices = Vec::new();
        for i in 0..NUM {
            let absolute_index = loop {
                let index = max_index(&line[search_range.clone()]);
                let absolute_index = index + search_range.start;
                let number_of_digits_after = line.len() - 1 - absolute_index;
                let number_of_digits_need = NUM - 1 - i;
                if number_of_digits_after < number_of_digits_need {
                    search_range.end -= 1;
                    continue;
                }
                break absolute_index;
            };
            search_range.start = absolute_index + 1;
            search_range.end = line.len();
            indices.push(absolute_index);
        }
        let mut number = 0;
        for i in indices {
            let digit = line[i] - b'0';
            number = number * 10 + digit as u64;
        }
        println!("{}", number);
        answer += number;
    }
    println!("{}", answer);
}
