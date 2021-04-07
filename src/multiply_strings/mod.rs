use std::char;
use std::iter::FromIterator;

pub fn multiply_strings(num1: String, num2: String) -> String {
  if num1 == "0" || num2 == "0" {
    return String::from("0");
  }

  let mut partials: Vec<Vec<char>> = vec![Vec::new(); num1.chars().count()];

  let mut carry: u32;
  let base = 10;

  for (i, c1) in num1.chars().rev().enumerate() {
    carry = 0;

    for _ in 0..i {
      partials[i].push('0');
    }

    for c2 in num2.chars().rev() {
      let num1 = c1.to_digit(base).unwrap();
      let num2 = c2.to_digit(base).unwrap();

      let mut prod = num1 * num2 + carry;
      carry = prod / base;
      prod = prod % base;

      partials[i].push(char::from_digit(prod, base).unwrap());
    }

    if carry != 0 {
      partials[i].push(char::from_digit(carry, base).unwrap());
    }
  }

  let mut product: Vec<char> = Vec::new();

  carry = 0;

  for col in 0..partials[partials.len() - 1].len() {
    let mut col_sum = 0;
    for v in 0..partials.len() {
      if col < partials[v].len() {
        col_sum += partials[v][col].to_digit(base).unwrap();
      }
    }

    col_sum += carry;

    product.push(char::from_digit(col_sum % base, base).unwrap());

    carry = col_sum / base;
  }

  if carry != 0 {
    product.push(char::from_digit(carry, base).unwrap());
  }

  String::from_iter(product.iter().rev())
}

#[cfg(test)]
mod tests {
    use crate::{multiply_strings};

    #[test]
    fn simple() {
        assert_eq!(multiply_strings(String::from("123"), String::from("456")), String::from("56088"));
        assert_eq!(multiply_strings(String::from("9133"), String::from("0")), String::from("0"));
        assert_eq!(multiply_strings(String::from("999"), String::from("999")), String::from("998001"));
        assert_eq!(multiply_strings(String::from("6"), String::from("2")), String::from("12"));
    }
}
