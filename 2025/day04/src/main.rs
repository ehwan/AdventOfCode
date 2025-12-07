use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut board = input
        .trim()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    fn get(board: &Vec<Vec<u8>>, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 || x >= board[0].len() as i32 || y >= board.len() as i32 {
            return b'.';
        }
        return board[y as usize][x as usize];
    }

    let mut answer = 0;
    loop {
        let mut changed = false;
        for y in 0..board.len() {
            for x in 0..board[0].len() {
                if board[y][x] == b'.' {
                    continue;
                }
                let mut at_count = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        let n = get(&board, nx, ny);
                        if n == b'@' {
                            at_count += 1;
                        }
                    }
                }
                if at_count < 4 {
                    changed = true;
                    board[y][x] = b'.';
                    answer += 1;
                }
            }
        }
        if !changed {
            break;
        }
    }
    println!("{}", answer);
}
