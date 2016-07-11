#[macro_use]
extern crate nom;
extern crate rand;

mod parser;
mod roll;
mod types;

pub use parser::dice_roll;
pub use roll::{
  roll_dice, roll_fate_dice,
  roll_multiple_die, roll_multiple_fate_die,
};
pub use types::{DiceMutation, DiceSize, Token};
