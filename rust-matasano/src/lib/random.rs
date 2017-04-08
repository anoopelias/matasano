use rand::Rng;
use rand::os::OsRng;

use lib::utils;

pub struct Random {
    rng: Box<Rng>
}

impl Random {
    pub fn new() -> Random {
        let rng = OsRng::new().unwrap();
        Random { rng: Box::new(rng) }
    }

    pub fn rand(&mut self) -> i32 {
        let rand_u32 = self.rng.next_u32();
        (rand_u32 & 0x7fffffff) as i32
    }

    pub fn rand_range(&mut self, from: &i32, to: &i32) -> i32 {

        assert!(to > from);

        let range = to - from;
        let bits = utils::bits_in_num(&range);
        let max_num = (1 << bits) - 1;

        let rand;
        loop {
            let num = self.rand();
            if (num & max_num) < range {
                rand = num & max_num;
                break;
            }
        }

        *from + rand
    }

    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_rand() {
        let mut random = Random::new();
        random.rand();
    }

    #[test]
    fn test_random_rand_range() {
        let mut random = Random::new();
        for _ in 0..100 {
            let rand = random.rand_range(&10, &20);
            assert!(rand >= 10);
            assert!(rand < 20);
        }
    }
}

