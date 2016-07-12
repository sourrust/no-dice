use nom::{
  digit,
  Err, ErrorKind,
  IResult, Needed,
};

use std::str;
use std::str::FromStr;

use types::{DiceMutation, DiceSize, DiceRoll, Token, Modifier};

macro_rules! satisfy {
  ($input: expr, $is_character: expr) => (
    if $input.is_empty() {
      IResult::Incomplete(Needed::Size(1))
    } else if $is_character($input[0]) {
      IResult::Done(&$input[1..], $input[0])
    } else {
      IResult::Error(Err::Position(ErrorKind::Alpha, $input))
    }
  );
}

fn number<T>(input: &[u8]) -> IResult<&[u8], T>
 where T: FromStr {
  map_res!(input,
    map_res!(digit, str::from_utf8),
    str::parse
  )
}

#[inline]
fn is_fate_dice(byte: u8) -> bool {
  byte == b'F' || byte == b'f'
}

named!(fate_dice <&[u8], u8>, satisfy!(is_fate_dice));

named!(size <&[u8], DiceSize>,
  alt!(
    fate_dice => { |_| DiceSize::Fate } |
    number    => { DiceSize::Number }
  )
);

named!(dice_mutator <&[u8], DiceMutation>,
  alt!(
    preceded!(char!('d'), number) => { DiceMutation::DropLowest } |
    preceded!(char!('k'), number) => { DiceMutation::KeepHighest }
  )
);

fn dice_roll_token(input: &[u8],
                   is_negative: bool,
                   dice_count: usize)
                   -> IResult<&[u8], Token> {
  chain!(input,
    dice_size: preceded!(char!('d'), size) ~
    mutators: many0!(dice_mutator),
    || {
      let mut drop_dice = 0;
      let mut keep_dice = 0;

      for mutator in mutators {
        match mutator {
          DiceMutation::Drop(num) => drop_dice += num,
          DiceMutation::Keep(num) => keep_dice += num,
        }
      }

      Token::DiceRoll(DiceRoll {
        is_negative: is_negative,
        count: dice_count,
        size: dice_size,
        keep: keep_dice,
        drop: drop_dice,
      })
    }
  )
}

named!(pub dice_roll <&[u8], Vec<Token> >, many1!(token_parser));

named!(token_parser <&[u8], Token>,
  chain!(
    first_token: token ~
    peek_char: complete!(
      peek!(
        map!(arithmetic_sign, |sign| sign == b'*' || sign == b'/')
      )
    )? ~
    result: cond!(peek_char.unwrap_or(false),
      apply!(higher_order, &first_token)),
    || {
      result.unwrap_or(first_token)
    }
  )
);

named!(token <&[u8], Token>,
  chain!(
    is_negative: map!(
      opt!(arithmetic_sign),
      |sign: Option<u8>| sign.unwrap_or(b'+') == b'-'
    ) ~
    num: map!(opt!(number), |value: Option<usize>| value.unwrap_or(1)) ~
    result: apply!(dice_roll_token, is_negative, num)?,
    || {
      result.unwrap_or(Token::Modifier(Modifier {
        is_negative: is_negative,
        number: num as u16,
      }))
    }
  )
);

fn higher_order<'a>(input: &'a [u8],
                    first_token: &Token)
                    -> IResult<&'a [u8], Token> {
  match pair!(input, arithmetic_sign, token) {
    IResult::Error(error)                  => IResult::Error(error),
    IResult::Incomplete(need)              => IResult::Incomplete(need),
    IResult::Done(i, (sign, second_token)) => {
      let token_result = if sign == b'*' {
        Token::Multiplication(Box::new(first_token.clone()),
                              Box::new(second_token))
      } else {
        Token::Division(Box::new(first_token.clone()),
                        Box::new(second_token))
      };

      IResult::Done(i, token_result)
    }
  }
}

#[inline]
fn is_arithmetic_sign(byte: u8) -> bool {
  byte == b'*' || byte == b'+' || byte == b'-' || byte == b'/'
}

named!(arithmetic_sign <&[u8], u8>, satisfy!(is_arithmetic_sign));
