use std::{collections::BTreeSet, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let points = input
        .trim()
        .lines()
        .map(|line| {
            let nums = line
                .trim()
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (nums[0], nums[1], nums[2])
        })
        .collect::<Vec<_>>();

    let mut dists = Vec::with_capacity(points.len() * (points.len() - 1) / 2);
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let diff0 = points[i].0 - points[j].0;
            let diff1 = points[i].1 - points[j].1;
            let diff2 = points[i].2 - points[j].2;
            dists.push((diff0 * diff0 + diff1 * diff1 + diff2 * diff2, i, j));
        }
    }
    dists.sort();

    let mut parent = Vec::with_capacity(points.len());
    for i in 0..points.len() {
        parent.push(i);
    }

    fn get_root(parent: &Vec<usize>, i: usize) -> usize {
        let mut root = i;
        while parent[root] != root {
            root = parent[root];
        }
        root
    }
    fn is_one_circuit(parent: &mut Vec<usize>, len: usize) -> bool {
        let mut roots = BTreeSet::new();
        for i in 0..len {
            let root = get_root(parent, i);
            parent[i] = root;
            roots.insert(root);
            if roots.len() > 1 {
                return false;
            }
        }
        roots.len() == 1
    }

    for &(_, i, j) in dists.iter() {
        let before = is_one_circuit(&mut parent, points.len());
        let root_i = get_root(&parent, i);
        let root_j = get_root(&parent, j);
        parent[root_j] = root_i;
        let after = is_one_circuit(&mut parent, points.len());

        if !before && after {
            println!("{}", points[i].0 * points[j].0);
            break;
        }
    }
}
