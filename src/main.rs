use rand::{rngs::SmallRng, Rng, SeedableRng};

fn main() {
    let mut coin = Coin::new();
    for n in 2..32768 {
        test(&mut coin, n);
    }
}

const TIMES: u32 = 100000;

fn test(coin: &mut Coin, n: u32) {
    println!("--- n = {} ---", n);
    let mut total = 0;
    for _ in 0..TIMES {
        let (_, cnt) = coin_rand_optimized(coin, n);
        total += cnt;
    }
    let avg = total as f64 / TIMES as f64;
    let e = calc_expectation(n);
    let delta = (avg - e).abs();
    println!("average: {}, expectation: {}, delta: {}", avg, e, delta);
}

fn calc_expectation(n: u32) -> f64 {
    let t = n - 1;
    let bit_cnt = 32 - t.leading_zeros();
    let mut sum = 0;
    let mut mask = 1;
    for i in (2..=bit_cnt).rev() {
        if t & mask == 0 {
            sum += mask * i;
        }
        mask <<= 1;
    }
    return bit_cnt as f64 + sum as f64 / n as f64;
}

fn coin_rand_optimized(coin: &mut Coin, n: u32) -> (u32, u32) {
    let n = n - 1;
    let bit_cnt = 32 - n.leading_zeros();
    let mut cnt = 0u32;
    'outer: loop {
        let mut res = 0;
        let mut less = false;
        for i in (0..bit_cnt).rev() {
            cnt += 1;
            let b = coin.flip();
            if !less {
                let mask = 1 << i;
                if n & mask != 0 {
                    if !b {
                        less = true;
                    }
                } else if b {
                    continue 'outer;
                }
            }
            res |= (b as u32) << i;
        }
        return (res, cnt);
    }
}

fn _coin_rand_unoptimized(coin: &mut Coin, n: u32) -> (u32, u32) {
    let n = n - 1;
    let bit_cnt = 32 - n.leading_zeros();
    let mut cnt = 0u32;
    loop {
        let mut res = 0;
        for i in 0..bit_cnt {
            cnt += 1;
            res |= (coin.flip() as u32) << i;
        }
        if res <= n {
            return (res, cnt);
        }
    }
}

struct Coin(SmallRng);

impl Coin {
    fn new() -> Coin {
        Coin(SmallRng::from_entropy())
    }

    fn flip(&mut self) -> bool {
        self.0.gen()
    }
}
