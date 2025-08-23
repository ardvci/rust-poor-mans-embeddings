use std::num;

fn calculate_euclidian_distance(
    v1: &Vec<f32>,
    v2: &Vec<f32>,
) {
    if v1.len() != v2.len() {
        panic!("v1 and v2 must have the same length");
    }
    else {
        let mut total:f32 =0.0;
        for (x, y) in v1.iter().zip(v2.iter()) {
            total += ((x - y).abs()).powf(2.0);
        }

        println!("Euclidian distance: {}", total.sqrt());
    }


}

fn calculate_manhattan_distance(
    v1: &[f32],
    v2: &[f32],
) -> f32 {
    assert_eq!(v1.len(), v2.len(), "vectors must have the same length");
    let distance: f32 = v1.iter().zip(
        v2.iter()
    ).map(|(x,y)| (x-y).abs()).sum();
    println!("manhattan distance: {}", distance);
    distance
}

fn calculate_minkowski_distance(
    v1: &[f32],
    v2: &[f32],
    p: i32
) -> f32 {
    assert_eq!(v1.len(), v2.len(), "vectors must have the same length");
    assert!(p > 0, "p must be > 0");
    let distance: f32 = v1.iter().zip(v2.iter()
    ).map(
        |(x,y)| (x - y).abs().powi(p)
    ).sum();
    let pow_distance = distance.powf(1.0/p as f32);
    println!("minkowski distance: {}", pow_distance);
    pow_distance
}


fn calculate_chebyshev_distance(
    v1: &[f32],
    v2: &[f32]
) -> f32 {
    assert_eq!(v1.len(), v2.len(), "vectors must have the same length");
    let mut distance:f32 =0.0;
    for (x, y) in v1.iter().zip(v2.iter()) {
        let diff = (x - y).abs();
        if diff > distance {
            distance = diff;
        }
    }
    println!("chebyshev distance: {}", distance);
    distance
}

fn calculate_canberra_distance(
    v1: &[f32],
    v2: &[f32]
) -> f32{
    assert_eq!(v1.len(), v2.len(), "vectors must have the same length");
    let mut total:f32 =0.0;
    for (x,y) in v1.iter().zip(v2.iter()) {
        if x.abs() + y.abs() > 0.0 {
            total += (x-y).abs() / (x.abs() + y.abs());
        }
    }
    println!("Canberra distance: {}", total);
    total
}

fn calculate_hamming_distance(
    v1: &[f32],
    v2: &[f32]
) -> f32 {
    assert_eq!(v1.len(), v2.len(), "vectors must have the same length");
    let mut counter: f32 = 0.0;
    for (x,y) in v1.iter().zip(v2.iter()) {
        if x != y {
            counter += 1.0;
        }
    }
    counter
}

fn calculate_cosine_similarity(
    v1: &[f32],
    v2: &[f32]
) -> f32 {
    assert_eq!(v1.len(), v2.len(), "vectors must have the same length");
    let sum_of_multiplies: f32 = v1.iter().zip(v2.iter()).map(
        |(x,y)| (x * y)
    ).sum();

    let divider_x: f32 = v1.iter().map(
        |x| x.powi(2)
    ).sum::<f32>().sqrt();

    let divider_y: f32 = v2.iter().map(
        |y| y.powi(2)
    ).sum::<f32>().sqrt();

    let result = sum_of_multiplies / (divider_x * divider_y);

    println!("Cosine similarity: {}", result);
    result
}





// fn main() {
//     let v1 = vec![0.0,0.0,1.0];
//     let v2 = vec![1.0,0.0,0.0];
//
//     calculate_euclidian_distance(&v1, &v2);
//     calculate_manhattan_distance(&v1, &v2);
//     calculate_minkowski_distance(&v1, &v2,1);
//     calculate_chebyshev_distance(&v1, &v2);
//     calculate_canberra_distance(&v1, &v2);
//     calculate_hamming_distance(&v1, &v2);
//     calculate_cosine_similarity(&v1, &v2);
// }
