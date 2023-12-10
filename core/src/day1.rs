pub fn cubebits(num1220: String) -> u64 {
    num1220.split('/')
            .map(|x| match x.parse::<u64>() {
                    Ok(x) => x,
                    Err(_) => 0
                })
            .fold(0, |xor, x| xor ^ x)
            .pow(3)
}
