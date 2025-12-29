pub fn moyenne(vec: Vec<i32>) -> f64 {
    if vec.is_empty() {
        return 0.0;
    }

    let somme : i32 = vec.iter().sum();
    somme as f64 / vec.len() as f64
}