# Demo Vectors Cliques (`demo-vectors-cliques`)

A Rust project comparing implementations of the Bron-Kerbosch algorithm for finding cliques on fixed-size vector graphs (`n=4, q=3, d=3`).

## Project Structure

* `src/main.rs` – Entry point and DHAT heap profiler hooks.

* `src/helpers.rs` – Shared utilities.

* `src/cliques_433_r.rs` – Classical Recursive version (system-stack based, vectors).

* `src/cliques_433_i.rs` – Iterative version (heap-stack based, vectors).

* `src/cliques_433_b.rs` – Optimized recursive version (BitSet based).

## Found Cliques Output

| Clique 1: 9 vectors | Clique 2: 9 vectors | 
| ----- | ----- | 
| `0000` | `0000` | 
| `0111` | `0111` | 
| `0222` | `0222` | 
| `1012` | `1021` | 
| `1120` | `1102` | 
| `1201` | `1210` | 
| `2021` | `2012` | 
| `2102` | `2120` | 
| `2210` | `2201` | 

## Benchmark & Heap Profiling Results (DHAT)

Tested in release mode (`cargo run --release`) with individual heap profile outputs:

### 1. Classical Recursive (`cliques_433_r`)

* **Execution Time:** ~11.58 ms

* **DHAT Total Allocation:** 41,632 bytes in 604 blocks

* **Peak Heap (`At t-gmax`):** **4,160 bytes** in 29 blocks

### 2. Iterative (`cliques_433_i`)

* **Execution Time:** ~9.77 ms

* **DHAT Total Allocation:** 45,952 bytes in 608 blocks

* **Peak Heap (`At t-gmax`):** **7,168 bytes** in 48 blocks

### 3. Bitset-Optimized (`cliques_433_b`)

* **Execution Time:** **~3.15 ms** (Fastest)

* **DHAT Total Allocation:** **1,792 bytes** in 8 blocks (Minimal allocation)

* **Peak Heap (`At t-gmax`):** **1,648 bytes** in 4 blocks (Lowest peak memory)

## Performance & Analysis Summary

1. **BitSet Version (`cliques_433_b`)** is the absolute leader in both execution speed (~3.5x faster than the recursive vector version) and memory efficiency (over 2.5x less peak memory and only 8 total heap allocations). This performance gain stems from eliminating dynamic heap vector allocations per recursion step and relying on fast bitwise operations on fixed `[u64; 2]` arrays.

2. **Iterative Vector Version (`cliques_433_i`)** runs slightly faster in execution time (9.77 ms vs. 11.58 ms) compared to the classical recursive implementation. However, it consumes higher peak memory (7,168 bytes vs. 4,160 bytes) due to storing full vector instances inside the heap-allocated stack frame structures.

## How to Run & Benchmark

1. Clone the repository:

   ```bash
   git clone https://github.com/lnlbg/demo--vectors-cliques.git
   cd demo--vectors-cliques
   ```

2. Run in release mode for accurate timing and memory profiling:

   ```bash
   cargo run --release
   ```

## License

Licensed under the [MIT License](LICENSE) or [Apache-2.0 License](LICENSE).