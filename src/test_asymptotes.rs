use VEBTree;

use std::time::{Instant, Duration};

#[test]
fn test_insert_asymptote(){
    let num_runs = 500;
    let mut test_n_values: Vec<usize> = Vec::with_capacity(num_runs);
    for i in (0..num_runs) {
        test_n_values.push((i+1)*10_usize.pow(3));
    }
    let mut test_results: Vec<f64> = Vec::with_capacity(num_runs);
    for i in 0..test_n_values.len() {
        let mut test_tree = VEBTree::new(i);
        let start = Instant::now();
        for j in 0..i {
            if j % 2 == 0 {
                test_tree.insert(j);
            }
        }
        let elapsed = start.elapsed();
        test_results.push(convert_elapsed_to_nanosec(elapsed));
    }
    for i in 0..test_results.len() {
        test_results[i] = test_results[i].log2().log2();
    }
    let corr = calculate_correlation_coefficient(
        &test_results[0..test_results.len()],
        &test_n_values[0..test_n_values.len()]
    );
    println!("corr: {:?}", corr);
    assert!(corr > 0.70);
}

#[test]
fn test_delete_asymptote(){
    let num_runs = 500;
    let mut test_n_values: Vec<usize> = Vec::with_capacity(num_runs);
    for i in (0..num_runs) {
        test_n_values.push((i+1)*10_usize.pow(3));
    }
    let mut test_results: Vec<f64> = Vec::with_capacity(num_runs);
    for i in 0..test_n_values.len() {
        let mut test_tree = VEBTree::new(i);
        for j in 0..i {
            if j % 2 == 0 {
                test_tree.insert(j);
            }
        }
        let start = Instant::now();
        for j in 0..i {
            if j % 2 == 0 {
                test_tree.delete(j);
            }
        }
        let elapsed = start.elapsed();
        test_results.push(convert_elapsed_to_nanosec(elapsed));
    }
    for i in 0..test_results.len() {
        test_results[i] = test_results[i].log2().log2();
    }
    let corr = calculate_correlation_coefficient(
        &test_results[0..test_results.len()],
        &test_n_values[0..test_n_values.len()]
    );
    println!("corr: {:?}", corr);
    assert!(corr > 0.70);
}

#[test]
fn test_findnext_asymptote(){
    let num_runs = 500;
    let mut test_n_values: Vec<usize> = Vec::with_capacity(num_runs);
    for i in (0..num_runs) {
        test_n_values.push((i+1)*10_usize.pow(3));
    }
    let mut test_results: Vec<f64> = Vec::with_capacity(num_runs);
    for i in 0..test_n_values.len() {
        let mut test_tree = VEBTree::new(i);
        for j in 0..i {
            if i % 2 == 0 {
                test_tree.insert(j);
            }
        }
        let start = Instant::now();
        for j in 0..i/2 {
            test_tree.findnext(j*2);
        }
        let elapsed = start.elapsed();
        test_results.push(convert_elapsed_to_nanosec(elapsed));
    }
    for i in 0..test_results.len() {
        test_results[i] = test_results[i].log2().log2();
    }
    let corr = calculate_correlation_coefficient(
        &test_results[0..test_results.len()],
        &test_n_values[0..test_n_values.len()]
    );
    println!("corr: {:?}", corr);
    assert!(corr > 0.70);
}


#[test]
fn test_findprev_asymptote(){
    let num_runs = 500;
    let mut test_n_values: Vec<usize> = Vec::with_capacity(num_runs);
    for i in (0..num_runs) {
        test_n_values.push((i+1)*10_usize.pow(3));
    }
    let mut test_results: Vec<f64> = Vec::with_capacity(num_runs);
    for i in 0..test_n_values.len() {
        let mut test_tree = VEBTree::new(i);
        for j in 0..i {
            if i % 2 == 0 {
                test_tree.insert(j);
            }
        }
        let start = Instant::now();
        for j in 0..i/2 {
            test_tree.findprev(j*2);
        }
        let elapsed = start.elapsed();
        test_results.push(convert_elapsed_to_nanosec(elapsed));
    }
    for i in 0..test_results.len() {
        test_results[i] = test_results[i].log2().log2();
    }
    let corr = calculate_correlation_coefficient(
        &test_results[0..test_results.len()],
        &test_n_values[0..test_n_values.len()]
    );
    println!("corr: {:?}", corr);
    assert!(corr > 0.70);
}







fn convert_elapsed_to_nanosec(elapsed: Duration) -> f64 {
    return ((elapsed.as_secs() as f64)*1_000_000_000.0)
        + ( elapsed.subsec_nanos() as f64);
}

fn calculate_correlation_coefficient(time: &[f64], size: &[usize]) -> f64 {
    assert_eq!(time.len(), size.len());
    let mut x: Vec<f64> = Vec::with_capacity(time.len());
    let mut y: Vec<f64> = Vec::with_capacity(time.len());
    for i in 0..time.len() {
        x.push(size[i] as f64);
        y.push(time[i]);
    }
    let mut xy: Vec<f64> = Vec::with_capacity(x.len());
    let mut x_2: Vec<f64> = Vec::with_capacity(x.len());
    let mut y_2: Vec<f64> = Vec::with_capacity(x.len());

    for i in 0..x.len() {
        xy.push(y[i]*x[i]);
        x_2.push(x[i].powf(2.0));
        y_2.push(y[i].powf(2.0));
    }
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = xy.iter().sum();
    let sum_x_2: f64 = x_2.iter().sum();
    let sum_y_2: f64 = y_2.iter().sum();
    println!("sum_x: {:?}, sum_y={:?}, sum_xy={:?}, sum_x_2={:?}, sum_y_2={:?}",
             sum_x,
             sum_y,
             sum_xy,
             sum_x_2,
             sum_y_2
    );
    let numerator: f64 = (x.len() as f64)*sum_xy - (sum_x)*(sum_y);
    let denominator_1: f64 =(x.len() as f64)*sum_x_2 - sum_x.powf(2.0);
    let denominator_2: f64 =(x.len() as f64)*sum_y_2 - sum_y.powf(2.0);
    let denominator = (denominator_1*denominator_2).sqrt();
    println!("Numerator: {:?}, Denominator: {:?}", numerator, denominator);
    return numerator/denominator;
}
