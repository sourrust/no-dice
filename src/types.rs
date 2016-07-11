#[derive(Debug, Clone)]
pub enum DiceSize {
  Fate,
  Number(u16),
}

pub enum DiceMutation {
  Drop(u32),
  Keep(u32),
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
