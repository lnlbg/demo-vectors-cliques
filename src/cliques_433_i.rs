use crate::helpers::{MIN_CLIQUE_SIZE, init_set, is_connected, print_clique};

struct StackFrame {
    r: Vec<[i32; 4]>,
    p: Vec<[i32; 4]>,
    x: Vec<[i32; 4]>,
}

pub fn bron_kerbosch_main() {
    let mut initial_p: Vec<[i32; 4]> = Vec::new();
    init_set(&mut initial_p);

    let initial_r: Vec<[i32; 4]> = Vec::new();
    let initial_x: Vec<[i32; 4]> = Vec::new();

    let mut stack = Vec::new();
    stack.push(StackFrame {
        r: initial_r,
        p: initial_p,
        x: initial_x,
    });

    while let Some(mut frame) = stack.pop() {
        if frame.r.len() + 2 + frame.p.len() < MIN_CLIQUE_SIZE {
            continue;
        }

        if frame.p.is_empty() {
            if frame.x.is_empty() {
                print_clique(&frame.r);
            }
            continue;
        }

        let mut s: Vec<[i32; 4]> = Vec::new();
        for &word in &frame.p {
            if !is_connected(&frame.p[0], &word) {
                s.push(word);
            }
        }

        for word in s {
            let mut new_p: Vec<[i32; 4]> = Vec::new();
            for &word_p in &frame.p {
                if is_connected(&word, &word_p) {
                    new_p.push(word_p);
                }
            }

            let mut new_x: Vec<[i32; 4]> = Vec::new();
            for &word_x in &frame.x {
                if is_connected(&word, &word_x) {
                    new_x.push(word_x);
                }
            }

            let mut next_r = frame.r.clone();
            next_r.push(word);

            frame.p.retain(|&val| val != word);
            frame.x.push(word);

            stack.push(StackFrame {
                r: next_r,
                p: new_p,
                x: new_x,
            });
        }
    }
}
