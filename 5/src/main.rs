use std::io;
use std::io::{Write};

type Jump = i64;
type Jumps = std::collections::VecDeque<Jump>;


fn main() {
    let mut j = Jumps::new();
    parse_stdin(&mut j);
    // println!("{:?}", j);
    let i = jumper_part_two(&mut j);
    println!("result: {}", i);
}

#[inline]
fn out(j: &Jumps, i: i64) -> bool {
    const min: i64 = 0;
    let max: i64 = j.len() as i64;
    i < min || i >= max
}

#[inline]
fn jumper_part_one(j: &mut Jumps) -> i64 {
    let mut i: i64 = 0;
    let show = j.len() < 10;
    if show {
        println!("\nindex: {}\n{:?}", i, j);
    }
    let mut counter: i64 = 0;
    while !out(&j, i) {
        counter += 1;
        let eye: usize = i as usize;
        i += j[eye];
        j[eye] += 1;
        if show {
            println!("\nindex: {}\n{:?}", i, j);
        }
    }
    counter
}

#[inline]
fn jumper_part_two(j: &mut Jumps) -> i64 {
    let mut i: i64 = 0;
    let show = j.len() < 10;
    if show {
        println!("\nindex: {}\n{:?}", i, j);
    }
    let mut counter: i64 = 0;
    while !out(&j, i) {
        counter += 1;
        let eye: usize = i as usize;
        i += j[eye];
        let mut augment_jeye = 1;
        if j[eye] >=3 {
            augment_jeye = -1;
        }
        j[eye] += augment_jeye;
        if show {
            println!("\nindex: {}\n{:?}", i, j);
        }
    }
    counter
}

fn parse_stdin(j: &mut Jumps) {
    std::io::stdout().flush().expect("flushing stdin fail");
    loop {
        let mut buf = String::new();
        let op = io::stdin().read_line(&mut buf);
        match op {
            Ok(x) => {
                if x == 0 {
                    break
                }
                let el: Jump = buf.trim().parse().expect("is that an integer?");
                j.push_back(el);
            },
            Err(_) => break,
        };
    }
}
