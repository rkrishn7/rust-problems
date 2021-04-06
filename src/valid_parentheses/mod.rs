enum BracketType {
  Left,
  Right,
}

pub fn is_valid(s: String) -> bool {
  let mut open_parens: Vec<char> = Vec::new();

  for c in s.chars() {
      let mut bt: Option<BracketType> = None;

      match c {
          '(' | '[' | '{' => { 
              open_parens.push(c);
              bt = Some(BracketType::Left);
          },
          ')' | ']' | '}' => {
              bt = Some(BracketType::Right)
          },
          _ => {},
      }

      if let Some(BracketType::Right) = bt {
          if let Some(o) = open_parens.pop() {
              match c {
                  ')' => {
                      if o != '(' { return false; }
                   },
                  '}' => {
                      if o != '{' { return false; }
                  },
                  ']' => {
                      if o != '[' { return false; }
                  },
                  _ => {},
              }
          } else { return false; }
      }
  }

  open_parens.len() == 0
}

#[cfg(test)]
mod tests {
    use crate::{check_parentheses};

    #[test]
    fn simple() {
        assert_eq!(check_parentheses(String::from("()")), true);
        assert_eq!(check_parentheses(String::from("()[]{}")), true);
        assert_eq!(check_parentheses(String::from("(]")), false);
        assert_eq!(check_parentheses(String::from("([)]")), false);
        assert_eq!(check_parentheses(String::from("{[]}")), true);
    }
}
