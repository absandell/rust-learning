use std::time::{Duration, SystemTime};

fn main() {
    let n = 40;
    let reps = 1;

    let mut results: Vec<u128> = Vec::with_capacity(reps);
    let mut i = 0;
    let mut result = 0;
    while i < reps {
        if i == 0{
            let now = SystemTime::now();
            result = nth_fibonacci_iterative(n);
            match now.elapsed() {
                Ok(elapsed) => {
                    results.push(elapsed.as_micros());
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        }else {
            let now = SystemTime::now();
            nth_fibonacci_iterative(n);
            match now.elapsed() {
                Ok(elapsed) => {
                    results.push(elapsed.as_micros());
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        }
        i += 1;
    }
    let count:u128 = results.iter().sum();
    let avg = count as f64 / reps as f64;
    println!("Result: {} - Avg Time for Iterative: {} microseconds", result, avg);

    // Recursive Performance Analysis
    let mut results: Vec<u128> = Vec::with_capacity(reps);
    let mut i = 0;
    let mut result = 0;
    while i < reps {
        if i == 0{
            let now = SystemTime::now();
            result = nth_fibonacci_recursive(n);
            match now.elapsed() {
                Ok(elapsed) => {
                    results.push(elapsed.as_micros());
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        }else {
            let now = SystemTime::now();
            nth_fibonacci_recursive(n);
            match now.elapsed() {
                Ok(elapsed) => {
                    results.push(elapsed.as_micros());
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        }
        i += 1;
    }
    let count:u128 = results.iter().sum();
    let avg = count as f64 / reps as f64;
    println!("Result: {} - Avg Time for Recursive : {} microseconds", result, avg);

    // Dynamic Performance Analysis
    let mut results: Vec<u128> = Vec::with_capacity(reps);
    let mut i = 0;
    let mut result = 0;
    while i < reps {
        if i == 0{
            let now = SystemTime::now();
            result = nth_fibonacci_dynamic(n as usize);
            match now.elapsed() {
                Ok(elapsed) => {
                    results.push(elapsed.as_micros());
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        }else {
            let now = SystemTime::now();
            nth_fibonacci_dynamic(n as usize);
            match now.elapsed() {
                Ok(elapsed) => {
                    results.push(elapsed.as_micros());
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        }
        i += 1;
    }
    let count:u128 = results.iter().sum();
    let avg = count as f64 / reps as f64;
    println!("Result: {} - Avg Time for Dynamic: {} microseconds", result, avg);

    // Optimized Performance Analysis
    let mut results: Vec<u128> = Vec::with_capacity(reps);
    let mut i = 0;
    let mut result = 0;
    while i < reps {
        if i == 0{
            let now = SystemTime::now();
            result = nth_fibonacci_optimized(n);
            match now.elapsed() {
                Ok(elapsed) => {
                    results.push(elapsed.as_micros());
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        }else {
            let now = SystemTime::now();
            nth_fibonacci_optimized(n);
            match now.elapsed() {
                Ok(elapsed) => {
                    results.push(elapsed.as_micros());
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        }
        i += 1;
    }
    let count:u128 = results.iter().sum();
    let avg = count as f64 / reps as f64;
    println!("Result: {} - Avg Time for Optimized: {} microseconds", result, avg);
}

// Time Complexity: O(n)
// Space Complexity: O(1)
fn nth_fibonacci_iterative(n: i64) -> i64{
    if n == 0 {
        return 0;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 1..n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

// Time Complexity: O(n^2)
// Space Complexity: O(n)
fn nth_fibonacci_recursive(n: i64) -> i64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return nth_fibonacci_recursive(n-1) + nth_fibonacci_recursive(n-2);
    }
}

// Time Complexity: O(n)
// Space Complexity: O(n)
fn nth_fibonacci_dynamic(n: usize) -> usize {
    let mut f: Vec<usize> = Vec::with_capacity(n+2);
    f.push(0);
    f.push(1);

    for i in 2..=n {
        f.push(f[i-1] + f[i-2]);
    }

    return f[n];
}

// Time Complexity: O(log(n))
// Space Complexity: O(1)
fn nth_fibonacci_optimized(n: i64) -> i64 {
    let mut F: Vec<Vec<i64>> = vec![vec![1,1], vec![1,0]];

    if n == 0 {
        return 0;
    }

    power_helper(&mut F, n-1);
    F[0][0]
}

fn power_helper(F: &mut Vec<Vec<i64>>, n: i64){
    if n == 0 || n == 1 {return};

    let M: Vec<Vec<i64>> = vec![vec![1,1], vec![1,0]];
    power_helper(F, n/2);
    multiply_helper(F, &F.clone());
    
    if n % 2 != 0{
        multiply_helper(F, &M);
    }
}

fn multiply_helper(F: &mut Vec<Vec<i64>>, M: &Vec<Vec<i64>>) {
    let x = F[0][0] * M[0][0] + F[0][1] * M[1][0];
    let y = F[0][0] * M[0][1] + F[0][1] * M[1][1];
    let z = F[1][0] * M[0][0] + F[1][1] * M[1][0];
    let w = F[1][0] * M[0][1] + F[1][1] * M[1][1];

    F[0][0] = x;
    F[0][1] = y;
    F[1][0] = z;
    F[1][1] = w;
}