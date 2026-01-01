use super::random::SplittableRandom;
use crate::collections::HugeLongArray;

pub trait Random {
    fn next_int(&mut self, origin: i32, bound: i32) -> i32;
    fn next_long(&mut self, origin: usize, bound: usize) -> usize;
}

pub struct ShuffleUtil;

impl ShuffleUtil {
    pub fn shuffle_array(data: &mut HugeLongArray, random: &mut dyn Random) {
        let size = data.size();
        if size < 2 {
            return;
        }
        for offset in 0..(size - 1) {
            let swap_with = random.next_long(offset, size);
            let temp = data.get(swap_with);
            let current = data.get(offset);
            data.set(swap_with, current);
            data.set(offset, temp);
        }
    }

    pub fn shuffle_slice(data: &mut [i32], random: &mut dyn Random) {
        if data.len() < 2 {
            return;
        }
        for offset in 0..(data.len() - 1) {
            let swap_with = random.next_int(offset as i32, data.len() as i32) as usize;
            data.swap(offset, swap_with);
        }
    }

    pub fn create_random_data_generator(random_seed: Option<u64>) -> SplittableRandom {
        SplittableRandom::with_seed(random_seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffle_slice_produces_permutation() {
        let mut data = vec![0, 1, 2, 3, 4, 5];
        let mut rng = SplittableRandom::with_seed(Some(42));
        ShuffleUtil::shuffle_slice(&mut data, &mut rng);
        let mut sorted = data.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn shuffle_array_produces_permutation() {
        let mut array = HugeLongArray::new(6);
        for i in 0..array.size() {
            array.set(i, i as i64);
        }
        let mut rng = SplittableRandom::with_seed(Some(99));
        ShuffleUtil::shuffle_array(&mut array, &mut rng);
        let mut sorted = array.to_vec();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![0, 1, 2, 3, 4, 5]);
    }
}
