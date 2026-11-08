use crate::helpers::{MIN_CLIQUE_SIZE, print_clique_indices};

type BitSet = [u64; 2];

struct Graph {
    vertices: Vec<[u8; 4]>,
    adjacency: Vec<BitSet>,
}

#[inline]
fn bit_set(set: &mut BitSet, index: usize) {
    set[index / 64] |= 1u64 << (index % 64);
}

#[inline]
fn bit_and(a: &BitSet, b: &BitSet) -> BitSet {
    [a[0] & b[0], a[1] & b[1]]
}

#[inline]
fn bit_diff(a: &BitSet, b: &BitSet) -> BitSet {
    [a[0] & !b[0], a[1] & !b[1]]
}

#[inline]
fn bit_count(set: &BitSet) -> u32 {
    set[0].count_ones() + set[1].count_ones()
}

#[inline]
fn bit_is_empty(set: &BitSet) -> bool {
    set[0] == 0 && set[1] == 0
}

#[inline]
fn bit_clear(set: &mut BitSet, index: usize) {
    set[index / 64] &= !(1u64 << (index % 64));
}

#[inline]
fn pop_first_bit(set: &mut BitSet) -> Option<usize> {
    if set[0] != 0 {
        let tz = set[0].trailing_zeros();
        set[0] &= set[0] - 1;
        Some(tz as usize)
    } else if set[1] != 0 {
        let tz = set[1].trailing_zeros();
        set[1] &= set[1] - 1;
        Some(64 + tz as usize)
    } else {
        None
    }
}

fn init_graph() -> Graph {
    let w0: [u8; 4] = [0, 0, 0, 0];
    let w1: [u8; 4] = [1, 1, 1, 0];
    let mut vertices = Vec::new();

    for i3 in 0..3 {
        for i2 in 0..3 {
            for i1 in 0..3 {
                for i0 in 0..3 {
                    let w = [i0 as u8, i1 as u8, i2 as u8, i3 as u8];
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
                        vertices.push(w);
                    }
                }
            }
        }
    }

    let n = vertices.len();
    let mut adjacency = vec![[0u64; 2]; n];

    for i in 0..n {
        for j in 0..n {
            if i != j {
                let mut diff = 0;
                for k in 0..4 {
                    if vertices[i][k] != vertices[j][k] {
                        diff += 1;
                    }
                }
                if diff >= 3 {
                    bit_set(&mut adjacency[i], j);
                }
            }
        }
    }

    Graph {
        vertices,
        adjacency,
    }
}

fn bron_kerbosch(r: &mut Vec<usize>, mut p: BitSet, mut x: BitSet, graph: &Graph) {
    if r.len() + 2 + (bit_count(&p) as usize) < MIN_CLIQUE_SIZE {
        return;
    }

    if bit_is_empty(&p) {
        if bit_is_empty(&x) {
            // Печатът вече се извиква централизирано от helpers.rs
            print_clique_indices(r, &graph.vertices);
        }
        return;
    }

    let first_v = {
        let mut temp = p;
        pop_first_bit(&mut temp).unwrap()
    };

    let s_bitset = bit_diff(&p, &graph.adjacency[first_v]);

    let mut temp_s = s_bitset;
    while let Some(v) = pop_first_bit(&mut temp_s) {
        r.push(v);

        bron_kerbosch(
            r,
            bit_and(&p, &graph.adjacency[v]),
            bit_and(&x, &graph.adjacency[v]),
            graph,
        );

        r.pop();

        bit_clear(&mut p, v);
        bit_set(&mut x, v);
    }
}

pub fn bron_kerbosch_main() {
    let graph = init_graph();
    let mut r = Vec::new();
    let n = graph.vertices.len();

    let mut p = [0u64; 2];
    for i in 0..n {
        bit_set(&mut p, i);
    }
    let x = [0u64; 2];

    bron_kerbosch(&mut r, p, x, &graph);
}
