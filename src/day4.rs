use md5;

pub fn day4() {
    let input = "yzbqklnj";
    find_key(input, 5);
    find_key(input, 6);
}

fn find_key(input: &str, num_zeros: usize) {
    for i in 1.. {
        if compute_hash(format!("{}{}", input, i), num_zeros) == "0".repeat(num_zeros){
            println!("The key is {}", i);
            break;
        }
    }
}

fn compute_hash(s: String, num_zeros: usize) -> String {
    let hash = format!("{:x}", md5::compute(s));
    String::from(&hash[0..num_zeros])
}
