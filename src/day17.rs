use rayon::prelude::*;
use std::sync::Mutex;

pub fn day17() {
    let coins = vec![
        11, 30, 47, 31, 32, 36, 3, 1, 5, 3, 32, 36, 15, 11, 46, 26, 28, 1, 19, 3,
    ];
    let target = 150;
    let count = Mutex::new(0);
    let min = Mutex::new(coins.len());

    (0usize..=1 << coins.len())
        .into_par_iter()
        .filter(|mask| {
            coins
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & 1 << i != 0)
                .map(|(_, coin)| coin)
                .sum::<usize>()
                == target
        })
        .for_each(|mask| {
            *count.lock().unwrap() += 1;

            let mut lock = min.lock().unwrap();
            let mask_count = count_bits(mask);
            if *lock > mask_count {
                *lock = mask_count;
            };
        });

    let min = *min.lock().unwrap();
    assert_eq!(*count.lock().unwrap(), 4372);
    assert_eq!(min, 4);

    let count = (0usize..=1 << coins.len())
        .into_par_iter()
        .filter(|&mask| {
            count_bits(mask) == min
                && coins
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask & 1 << i != 0)
                    .map(|(_, coin)| coin)
                    .sum::<usize>()
                    == target
        })
        .count();

    assert_eq!(count, 4);
}

#[inline]
fn count_bits(mut n: usize) -> usize {
    let mut count = 0;
    while n > 0 {
        count += 1;
        n &= n - 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17_test() {
        day17();
    }
}
