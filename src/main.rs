extern crate no_dice;
extern crate nom;

use std::env;
use nom::IResult;
use no_dice::{DiceSize, Token};

fn main() {
  let mut args = Vec::new();

  for arg in env::args().skip(1) {
    let mut bytes = arg.into_bytes();

    args.append(&mut bytes);
  }

  let result = no_dice::dice_roll(&args);

  if let IResult::Done(_, tokens) = result {
    for token in tokens {
      println!("{:?}", token);

      match token {
        Token::DiceRoll(dice) => {
          let roll = match dice.size {
            DiceSize::Number(size) => no_dice::roll_multiple_die(
                                        dice.count, size as i32),
            DiceSize::Fate         => no_dice::roll_multiple_fate_die(
                                        dice.count),
          };

          println!("{:?}", roll);
          println!("{}", roll.iter().fold(0, |x, y| x + y));
        }
        _                     => continue,
      }
    }
  } else {
    println!("{:?}", result);
  }
}
