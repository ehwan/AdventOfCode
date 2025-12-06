use std::{collections::BTreeSet, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut invalid_nums: Vec<BTreeSet<u64>> = vec![BTreeSet::new(); 11];
    {
        let mut x = 1;
        for i in 2..=10 {
            x = x * 10 + 1;
            invalid_nums[i].insert(x);
        }
    }
    fn precalculate_until(invalid_nums: &mut BTreeSet<u64>, max: u64, repeat: usize) {
        let current_max = *invalid_nums.last().unwrap_or(&0);
        if current_max >= max {
            return;
        }

        let current_max = current_max.to_string();
        let current_max_half = current_max[..current_max.len() / repeat]
            .parse::<u64>()
            .unwrap();

        for i in current_max_half + 1.. {
            let new_str = i.to_string().repeat(repeat);
            let new_num = match new_str.parse::<u64>() {
                Ok(x) => x,
                Err(_) => break, // out of range; break the loop
            };
            invalid_nums.insert(new_num);
            if new_num >= max {
                break;
            }
        }
    }

    let mut answer = 0;
    for line in input.trim().split(",") {
        println!("{}", line);
        let nums = line
            .trim()
            .split("-")
            .map(|x| {
                println!("{}", x);
                x.parse::<u64>().unwrap()
            })
            // .map(|x| .parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let mut unique_nums = BTreeSet::new();
        for i in 2..=10 {
            precalculate_until(&mut invalid_nums[i], nums[1], i);
            unique_nums.extend(invalid_nums[i].range(nums[0]..=nums[1]).copied());
        }
        let sum: u64 = unique_nums.into_iter().sum();
        answer += sum;
    }
    println!("{}", answer);
}
