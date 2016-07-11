use rand;
use rand::Rng;

fn roll_range(low: i32, high: i32) -> i32 {
  rand::thread_rng().gen_range(low, high)
}

#[inline]
pub fn roll_dice(size: i32) -> i32 {
  roll_range(1, size + 1)
}

#[inline]
pub fn roll_fate_dice() -> i32 {
  roll_range(-1, 2)
}

#[inline]
pub fn roll_multiple_die(number: usize, size: i32) -> Vec<i32> {
  let mut rolls = Vec::with_capacity(number);

  for _ in 0..number {
    rolls.push(roll_dice(size));
  }

  rolls
}

#[inline]
pub fn roll_multiple_fate_die(number: usize) -> Vec<i32> {
  let mut rolls = Vec::with_capacity(number);

  for _ in 0..number {
    rolls.push(roll_fate_dice());
  }

  rolls
}
