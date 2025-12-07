use std::io::Read;

fn simulate(board: &Vec<Vec<u8>>, cache: &mut Vec<Vec<u64>>, y: usize, x: usize) -> u64 {
    if cache[y][x] != 0 {
        return cache[y][x];
    }
    for y in y + 1..board.len() {
        if board[y][x] == b'^' {
            if cache[y][x] != 0 {
                return cache[y][x];
            } else {
                let left = if x > 0 {
                    simulate(board, cache, y, x - 1)
                } else {
                    0
                };
                let right = if x < board[y].len() - 1 {
                    simulate(board, cache, y, x + 1)
                } else {
                    0
                };
                cache[y][x] = left + right;
                return cache[y][x];
            }
        }
    }
    return 1;
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let board = input
        .trim()
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut cache = vec![vec![0; board[0].len()]; board.len()];
    let mut answer = 0;
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == b'S' {
                answer = simulate(&board, &mut cache, y, x);
                break;
            }
        }
    }
    println!("{}", answer);
}
