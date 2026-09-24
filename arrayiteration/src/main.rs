fn main() {
    let primes = [2,3,5,7,11,13,17,19];
    for prime in primes {
        println!("prime: {prime:?}");
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
}
