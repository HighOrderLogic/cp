use std::io::{Write, stdin, stdout};

fn ask(i: usize, j: usize) -> i32 {
  println!("? {} {}", i, j);
  stdout().flush().unwrap();
  let mut s = String::new();
  stdin().read_line(&mut s).unwrap();
  s.trim().parse().unwrap()
}

fn main() {
  let nums = vec![4, 8, 15, 16, 23, 42];

  let p12 = ask(1, 2);
  let p23 = ask(2, 3);
  let p34 = ask(3, 4);
  let p45 = ask(4, 5);

  let mut ans = [0; 6];

  for &x in &nums {
    for &y in &nums {
      if x * y == p12 {
        for &z in &nums {
          if y * z == p23 && x != y && x != z && y != z {
            ans[0] = x;
            ans[1] = y;
            ans[2] = z;
          }
        }
      }
    }
  }

  for &x in &nums {
    if ans[2] * x == p34 && x != ans[0] && x != ans[1] && x != ans[2] {
      ans[3] = x;
    }
  }

  for &x in &nums {
    if ans[3] * x == p45 && !ans[0..4].contains(&x) {
      ans[4] = x;
    }
  }

  for &x in &nums {
    if !ans.contains(&x) {
      ans[5] = x;
    }
  }

  print!("! ");
  let res = ans.iter().fold(String::new(), |s, v| s + " " + v.to_string().as_str());
  println!("{}", res);
}
