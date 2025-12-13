use std::{collections::HashMap, io::Read};

//[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

/*
Part 1
fn solve_impl(
    status: u32,
    buttons: &[u32],
    i: usize,
    cache: &mut HashMap<(u32, usize), usize>,
) -> usize {
    if i == buttons.len() {
        return if status == 0 { 0 } else { usize::MAX };
    }

    let case1 = solve_impl(status ^ buttons[i], buttons, i + 1, cache).saturating_add(1);
    let case2 = solve_impl(status, buttons, i + 1, cache);
    let answer = std::cmp::min(case1, case2);
    cache.insert((status, i), answer);
    answer
}
fn solve(status: u32, buttons: &[u32]) -> usize {
    let mut cache = HashMap::new();
    solve_impl(status, buttons, 0, &mut cache)
}
*/

fn solve_impl(
    buttons: &[Vec<u32>],
    counters: &mut Vec<u32>,
    i: usize,
    cache: &mut HashMap<(usize, Vec<u32>), usize>,
) -> usize {
    if i == buttons.len() {
        if counters.iter().all(|&c| c == 0) {
            return 0;
        } else {
            return usize::MAX;
        }
    }

    if let Some(cache) = cache.get(&(i, counters.clone())) {
        return *cache;
    }

    let max_count = buttons[i]
        .iter()
        .map(|&b| counters[b as usize])
        .min()
        .unwrap();
    let mut answer = usize::MAX;

    for &b in &buttons[i] {
        counters[b as usize] -= max_count;
    }
    let cur = solve_impl(buttons, counters, i + 1, cache).saturating_add(max_count as usize);
    answer = answer.min(cur);
    for c in (0..max_count).rev() {
        for &b in &buttons[i] {
            counters[b as usize] += 1;
        }
        let cur = solve_impl(buttons, counters, i + 1, cache).saturating_add(c as usize);
        answer = answer.min(cur);
    }

    cache.insert((i, counters.clone()), answer);

    answer
}
fn solve(buttons: Vec<Vec<u32>>, mut counters: Vec<u32>) -> usize {
    solve_impl(&buttons, &mut counters, 0, &mut HashMap::new())
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut answer = 0;
    for line in input.trim().lines() {
        let bracket_begin = line.find('[').unwrap();
        let bracket_end = line.find(']').unwrap();

        /*
        let mut status: u32 = 0;
        for (i, b) in line[bracket_begin + 1..bracket_end].bytes().enumerate() {
            if b == b'#' {
                status |= 1 << i;
            }
        }
        */

        let mut buttons = Vec::new();
        let mut s = &line[bracket_end + 1..];
        while let Some(paren_begin) = s.find('(') {
            let paren_end = s.find(')').unwrap();

            // let mut b = 0;
            let mut numbers = Vec::new();
            for d in s[paren_begin + 1..paren_end].split(',') {
                let d = d.parse::<u32>().unwrap();
                numbers.push(d);
                // b |= 1 << d;
            }
            buttons.push(numbers);

            s = &s[paren_end + 1..];
        }

        let brace_begin = line.find('{').unwrap();
        let brace_end = line.find('}').unwrap();
        let mut counters = Vec::new();
        for d in line[brace_begin + 1..brace_end].split(',') {
            counters.push(d.parse::<u32>().unwrap());
        }

        let s = solve(buttons, counters);
        println!("{}", s);
        answer += s;
    }
    println!("Answer: {}", answer);
}
