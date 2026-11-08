pub const MIN_CLIQUE_SIZE: usize = 9;

// За класическите вектори (рекурсивна и итеративна версия)
pub fn print_clique(q: &Vec<[i32; 4]>) {
    println!("- {} -", q.len() + 2);
    println!("0000");
    println!("0111");

    for i in 0..q.len() {
        for j in (0..=3).rev() {
            print!("{}", q[i][j]);
        }
        println!();
    }
}

// За BitSet версията, която работи с индекси и графични върхове ([u8; 4])
pub fn print_clique_indices(r: &[usize], vertices: &[[u8; 4]]) {
    println!("- {} -", r.len() + 2);
    println!("0000");
    println!("0111");

    for &idx in r {
        let w = vertices[idx];
        for j in (0..=3).rev() {
            print!("{}", w[j]);
        }
        println!();
    }
}

pub fn init_set(p: &mut Vec<[i32; 4]>) {
    let w0 = [0, 0, 0, 0];
    let w1 = [1, 1, 1, 0];

    for i3 in 0..3 {
        for i2 in 0..3 {
            for i1 in 0..3 {
                for i0 in 0..3 {
                    let w = [i0, i1, i2, i3];

                    let mut k = 0;
                    for j in 0..4 {
                        if w[j] != w0[j] {
                            k += 1;
                        }
                    }

                    let mut l = 0;
                    for j in 0..4 {
                        if w[j] != w1[j] {
                            l += 1;
                        }
                    }

                    if k >= 3 && l >= 3 {
                        p.push(w);
                    }
                }
            }
        }
    }
}

pub fn is_connected(w1: &[i32; 4], w2: &[i32; 4]) -> bool {
    let mut k = 0;
    for j in 0..4 {
        if w1[j] != w2[j] {
            k += 1;
        }
    }

    if k >= 3 {
        return true;
    }
    false
}
