use std::io::{BufWriter, Write, stdin, stdout};

fn solve(s: String) -> i32 {
  let cs = s.chars().collect::<Vec<_>>();
  let mut res = 0;
  let mut max_red = 0;

  for (i, c) in s.clone().chars().enumerate() {
    if i > 0 && i < s.len() - 1 {
      let prev = *cs.get(i - 1).unwrap();
      let next = *cs.get(i + 1).unwrap();

      let mut current_red = 0;
      if prev != c {
        current_red += 1;
      }
      if c != next {
        current_red += 1;
      }
      if prev != next {
        current_red -= 1;
      }

      if current_red > max_red {
        max_red = current_red;
      }
    }
    if i == 0 {
      res += 1;
    } else {
      res += i32::from(c != *cs.get(i - 1).unwrap());
    }
  }

  res - max_red
}

fn main() {
  let input = stdin();
  let mut out = BufWriter::new(stdout());
  let mut cnt_str = String::new();
  input.read_line(&mut cnt_str).unwrap();
  let n = cnt_str.trim_end().parse::<i32>().unwrap();
  for _ in 0..n {
    let mut n_str = String::new();
    input.read_line(&mut n_str).unwrap();
    let mut s = String::new();
    input.read_line(&mut s).unwrap();
    s = s.trim_end().to_owned();
    writeln!(out, "{}", solve(s)).unwrap();
  }
}

#[cfg(test)]
mod tests {
  use std::fmt::Display;

  use super::*;

  #[test]
  fn sample() {
    fn assert_output(s: impl Display, expected: i32) {
      assert_eq!(solve(s.to_string()), expected)
    }

    assert_output("abb", 2);
    assert_output("aab", 2);
    assert_output("abc", 2);
    assert_output("abaa", 1);
    assert_output("abba", 3);
    assert_output("eeeee", 1);
    assert_output("yyssee", 3);
    assert_output("abacaba", 5);
    assert_output("goodluckandhavefun", 16);
  }
}
