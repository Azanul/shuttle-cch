pub fn cubebits(num1220: String) -> i64 {
    num1220.split('/')
            .map(|x| x.parse::<i64>().unwrap_or_default())
            .fold(0, |xor, x| xor ^ x)
            .pow(3)
}
