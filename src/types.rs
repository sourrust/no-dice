#[derive(Debug, Clone)]
pub enum DiceSize {
  Fate,
  Number(u16),
}

pub enum DiceMutation {
  DropLowest(u32),
  DropHighest(u32),
  KeepLowest(u32),
  KeepHighest(u32),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum DiceValue {
  Highest,
  Lowest,
}

#[derive(Debug, Clone)]
pub struct Modifier {
  pub is_negative: bool,
  pub number: u16,
}

#[derive(Debug, Clone)]
pub struct DiceRoll {
  pub is_negative: bool,
  pub count: usize,
  pub size: DiceSize,
  pub keep: u32,
  pub drop: u32,
}

#[derive(Debug, Clone)]
pub enum Token {
  Modifier(Modifier),
  Multiplication(Box<Token>, Box<Token>),
  Division(Box<Token>, Box<Token>),
  DiceRoll(DiceRoll),
}
