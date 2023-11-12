use std::time::Instant;
use rand::Rng;
use std::thread;

fn generate_large_input(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let size_i32 = size as i32;
    (-size_i32..=size_i32).map(|_| rng.gen_range(-size_i32..=size_i32)).collect()
}

fn max_product_of_three_parallel(nums: &mut Vec<i32>) -> i128 {
    let len = nums.len();
    let num_threads = 4; // You can adjust the number of threads based on your system and workload

    // Calculate the chunk size for each thread
    let chunk_size = len / num_threads;

    // Create handles for each thread
    let handles: Vec<_> = (0..num_threads).map(|i| {
        let nums_chunk = nums[i * chunk_size..(i + 1) * chunk_size].to_vec();
        thread::spawn(move || {
            // Sort the vector chunk in ascending order
            let mut nums_chunk = nums_chunk;
            nums_chunk.sort();
            nums_chunk
        })
    }).collect();

    // Wait for all threads to finish and collect the sorted chunks
    let sorted_chunks: Vec<_> = handles.into_iter().map(|handle| handle.join().unwrap()).collect();

    // Merge sorted chunks if needed
    let mut merged_sorted = sorted_chunks.concat();
    merged_sorted.sort();

    // Calculate the product of the three largest elements.
    let product_with_max = match merged_sorted.len() {
        n if n >= 3 => {
            let last_index = n - 1;
            let potential_max_with_negatives =
                (merged_sorted[0] as i128) * (merged_sorted[1] as i128) * (merged_sorted[last_index] as i128);
            let potential_max_without_negatives =
                (merged_sorted[last_index - 2] as i128) * (merged_sorted[last_index - 1] as i128) * (merged_sorted[last_index] as i128);
            potential_max_with_negatives.max(potential_max_without_negatives)
        }
        _ => 0,
    };

    product_with_max
}



fn max_product_of_three_sequential(nums: &mut Vec<i32>) -> i128 {
    let len = nums.len();

    // Sort the vector in ascending order.
    nums.sort();

    // Calculate the product of the three largest elements.
    let product_with_max = if len >= 3 {
        let last_index = len - 1;
        let potential_max_with_negatives =
            (nums[0] as i128) * (nums[1] as i128) * (nums[last_index] as i128);
        let potential_max_without_negatives =
            (nums[last_index - 2] as i128) * (nums[last_index - 1] as i128) * (nums[last_index] as i128);
        potential_max_with_negatives.max(potential_max_without_negatives)
    } else {
        0
    };

    product_with_max
}


fn max_product_of_three_linear_scan(nums: &[i32]) -> i128 {
    let mut max1 = i32::min_value();
    let mut max2 = i32::min_value();
    let mut max3 = i32::min_value();
    let mut min1 = i32::max_value();
    let mut min2 = i32::max_value();

    for &num in nums {
        if num > max1 {
            max3 = max2;
            max2 = max1;
            max1 = num;
        } else if num > max2 {
            max3 = max2;
            max2 = num;
        } else if num > max3 {
            max3 = num;
        }

        if num < min1 {
            min2 = min1;
            min1 = num;
        } else if num < min2 {
            min2 = num;
        }
    }

    let product_with_max = (max1 as i128) * (max2 as i128) * (max3 as i128);
    let product_with_min = (max1 as i128) * (min1 as i128) * (min2 as i128);

    product_with_max.max(product_with_min)
}

fn max_product_of_three_threaded(nums: &[i32]) -> i128 {
    let num_threads = 4;
    let chunk_size = nums.len() / num_threads;

    let handles: Vec<_> = (0..num_threads).map(|i| {
        let nums_chunk: Vec<i32> = nums[i * chunk_size..(i + 1) * chunk_size].to_vec();
        let handle = thread::spawn(move || {
            let mut max1 = i32::min_value();
            let mut max2 = i32::min_value();
            let mut max3 = i32::min_value();
            let mut min1 = i32::max_value();
            let mut min2 = i32::max_value();

            for &num in &nums_chunk {
                if num > max1 {
                    max3 = max2;
                    max2 = max1;
                    max1 = num;
                } else if num > max2 {
                    max3 = max2;
                    max2 = num;
                } else if num > max3 {
                    max3 = num;
                }
            
                if num < min1 {
                    min2 = min1;
                    min1 = num;
                } else if num < min2 {
                    min2 = num;
                }
            }
            (max1, max2, max3, min1, min2)
        });
        handle
    }).collect();

    let mut max1 = i32::min_value();
    let mut max2 = i32::min_value();
    let mut max3 = i32::min_value();
    let mut min1 = i32::max_value();
    let mut min2 = i32::max_value();

    for handle in handles {
        let (local_max1, local_max2, local_max3, local_min1, local_min2) = handle.join().unwrap();
        max1 = max1.max(local_max1);
        max2 = max2.max(local_max2);
        max3 = max3.max(local_max3);
        min1 = min1.min(local_min1);
        min2 = min2.min(local_min2);
    }

    let product_with_max = (max1 as i128) * (max2 as i128) * (max3 as i128);
    let product_with_min = (max1 as i128) * (min1 as i128) * (min2 as i128);

    product_with_max.max(product_with_min)
}


fn main() {
    const SIZES: [usize; 14] = [10000, 50000, 100000, 500000, 1000000, 2000000, 4000000, 8000000, 16000000, 32000000, 64000000, 128000000, 256000000, 256000000000000000];

    
    for &input_size in SIZES.iter() {
        let input_data = generate_large_input(1000);

        let start = Instant::now();
        let mut input_data_sequential = input_data.clone();
        max_product_of_three_sequential(&mut input_data_sequential); 
        let duration = start.elapsed();
        println!("Sequential Execution Time (Size {}): {:?}", input_size, duration);

        let start = Instant::now();
        let mut input_data_parallel = input_data.clone();
        max_product_of_three_parallel(&mut input_data_parallel); 
        let duration = start.elapsed();
        println!("Parallel Execution Time (Size {}): {:?}", input_size, duration);

        let start = Instant::now();
        let mut input_data_linear_scan_threaded = input_data.clone();
        max_product_of_three_threaded(&mut input_data_linear_scan_threaded);
        let duration = start.elapsed();
        println!("Threaded Execution Time (Size {}): {:?}", input_size, duration);

        let mut input_data_linear_scan = input_data.clone();
        let start = Instant::now();
        max_product_of_three_linear_scan(&mut input_data_linear_scan);
        let duration = start.elapsed();
        println!("Linear Scan Execution Time (Size {}): {:?}\n", input_size, duration);
    }
}

