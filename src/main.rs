mod cliques_433_b;
mod cliques_433_i;
mod cliques_433_r;
mod helpers;

// Enable the DHAT allocator for heap profiling
#[global_allocator]
static ALLOCATOR: dhat::Alloc = dhat::Alloc;

fn main() {
    // Initialize the heap profiler
    let _profiler = dhat::Profiler::new_heap();

    let start_time_r = std::time::Instant::now();
    cliques_433_r::bron_kerbosch_main();
    let elapsed_r = start_time_r.elapsed();
    println!("Elapsed Time Recursive: {:?}", elapsed_r);

    let start_time_i = std::time::Instant::now();
    cliques_433_i::bron_kerbosch_main();
    let elapsed_i = start_time_i.elapsed();
    println!("Elapsed Time Iterative: {:?}", elapsed_i);

    let start_time_b = std::time::Instant::now();
    cliques_433_b::bron_kerbosch_main();
    let elapsed_b = start_time_b.elapsed();
    println!("Elapsed Time Bitset: {:?}", elapsed_b);
}
