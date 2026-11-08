use crate::helpers::{MIN_CLIQUE_SIZE, init_set, is_connected, print_clique};

fn bron_kerbosch(r: Vec<[i32; 4]>, mut p: Vec<[i32; 4]>, mut x: Vec<[i32; 4]>) {
    if r.len() + 2 + p.len() < MIN_CLIQUE_SIZE {
        return;
    }

    if p.is_empty() {
        if x.is_empty() {
            print_clique(&r);
        }
        return;
    }

    let mut s: Vec<[i32; 4]> = Vec::new();
    for &word in &p {
        if !is_connected(&p[0], &word) {
            s.push(word);
        }
    }

    for word in s {
        let mut new_p: Vec<[i32; 4]> = Vec::new();
        for &word_p in &p {
            if is_connected(&word, &word_p) {
                new_p.push(word_p);
            }
        }

        let mut new_x: Vec<[i32; 4]> = Vec::new();
        for &word_x in &x {
            if is_connected(&word, &word_x) {
                new_x.push(word_x);
            }
        }

        let mut next_r = r.clone();
        next_r.push(word);

        p.retain(|&val| val != word);
        x.push(word);

        bron_kerbosch(next_r, new_p, new_x);
    }
}

pub fn bron_kerbosch_main() {
    let r: Vec<[i32; 4]> = Vec::new();
    let mut p: Vec<[i32; 4]> = Vec::new();
    let x: Vec<[i32; 4]> = Vec::new();

    init_set(&mut p);
    bron_kerbosch(r, p, x);
}
