// FINAL SOLUTION TO DAY NINE

pub fn day_nine(file_contents: String) {
    let sequences = file_contents.lines().map(|x| {
        x.split_whitespace().map(|y| y.parse().unwrap()).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();
    let seqs: i32 = sequences.iter().map(|x| resequence(x)).sum();
    println!("{}", seqs)
}

fn resequence(seq: &Vec<i32>) -> i32 {
    let mut sum_vector: Vec<i32> = Vec::new();
    let mut difference_vector: Vec<i32>;
    let mut temp_vec = seq.clone();
    while !is_zeroed(&temp_vec) {
        difference_vector = Vec::new();
        for i in 0..temp_vec.len() - 1 {
            difference_vector.push(temp_vec[i + 1] - temp_vec[i]);
        }
        sum_vector.push(temp_vec[0]);
        temp_vec = difference_vector;
    }
    sum_vector.iter().enumerate().fold(0, |acc, x| acc + x.1 * i32::pow(-1, x.0 as u32))
} 

fn is_zeroed(vec: &Vec<i32>) -> bool {
    let mut dedupped = vec.clone();
    dedupped.dedup();
    let sum: i32 = dedupped.iter().sum();
    dedupped.len() == 1 && sum == 0
}
