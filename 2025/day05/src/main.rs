use std::io::Read;

struct RangesContainer {
    child: Vec<RangesContainer>,
    begin: u64,
    length: u64,
    filled: bool,
}

impl RangesContainer {
    pub fn new() -> RangesContainer {
        Self::new_child(0, u64::MAX)
    }
    fn new_child(begin: u64, length: u64) -> RangesContainer {
        if length == 0 {
            panic!("length cannot be 0")
        }
        RangesContainer {
            child: Vec::new(),
            begin,
            length,
            filled: false,
        }
    }
    fn create_child(&mut self) {
        if self.filled {
            panic!("cannot create child of filled node");
        }
        let half = self.length / 2;
        let left_node = Self::new_child(self.begin, half);
        let right_node = Self::new_child(self.begin + half, self.length - half);
        self.child = vec![left_node, right_node];
    }
    pub fn start(&self) -> u64 {
        self.begin
    }
    pub fn last(&self) -> u64 {
        self.begin + (self.length - 1)
    }
    pub fn insert(&mut self, start: u64, last: u64) {
        if self.filled {
            return;
        }
        if start == self.start() && last == self.last() {
            self.filled = true;
            self.child.clear();
            return;
        }
        if self.child.is_empty() {
            self.create_child();
        }
        if start <= self.child[0].last() {
            let l = last.min(self.child[0].last());
            self.child[0].insert(start, l);
        }
        if last >= self.child[1].start() {
            let s = start.max(self.child[1].start());
            self.child[1].insert(s, last);
        }
    }
    pub fn check(&self, i: u64) -> bool {
        if i < self.start() || i > self.last() {
            return false;
        }
        if self.filled {
            return true;
        }
        self.child.iter().any(|c| c.check(i))
    }
    pub fn count(&self) -> u64 {
        if self.filled {
            self.length
        } else {
            self.child.iter().map(|x| x.count()).sum()
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut ranges = RangesContainer::new();

    let mut answer = 0;
    for line in input.trim().lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains("-") {
            let mut nums = line.split("-").map(|x| x.parse::<u64>().unwrap());
            let Some(start) = nums.next() else {
                unreachable!()
            };
            let Some(last) = nums.next() else {
                unreachable!()
            };
            ranges.insert(start, last);
        } else {
            let i = line.parse::<u64>().unwrap();
            if ranges.check(i) {
                println!("YES");
                answer += 1;
            } else {
                println!("NO");
            }
        }
    }
    println!("{}", answer);
    println!("Count: {}", ranges.count());
}
