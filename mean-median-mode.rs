// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), 
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn main() {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    let mut v = vec![213,34,234,879,345,23,324,7843,34];
    v.sort();

    let mut total: i32 = 0;

    for num in &v {
        total += num;

        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mean: f32 = total as f32/v.len() as f32;

    println!("The mean is {}", mean);

    let median = v[v.len()/2];

    println!("The median is {:?}", median);

    let mut k = 0;
    let mut v = 0;

    for (key, value) in &map {
        if value > &v {
            k = **key;
            v = *value;
        }
    }

    println!("The mode is {}", k);
}
