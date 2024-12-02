pub mod template;

#[derive(Debug)]
pub enum SplitResult {
  Result(Vec<SplitResult>),
  Value(String)
}

// Recursively split a string, pulling delimiters off the input delimiters list from the back
pub fn supersplit(input: &str, delims: &mut Vec<&str>) -> SplitResult {
  let result: SplitResult;
  if delims.is_empty() {
    result = SplitResult::Value(input.to_string());
  } else {
    let mut output = vec![];
    match delims.pop() {
      Some(delim) => {
        for part in input.split(delim) {
          output.push(supersplit(part, &mut delims.to_vec()));
        }
      }
      None => {}
    }

    result = SplitResult::Result(output);
  }
  result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supersplit() {
      let result = supersplit("some crazy,string bro", &mut vec![" ", ","]);
      let matches = match result {
        SplitResult::Result(r) => match &r[..] {
          [SplitResult::Result(first), SplitResult::Result(second)] => match [&first[..], &second[..]] {
            [[SplitResult::Value(x), SplitResult::Value(y)], [SplitResult::Value(z), SplitResult::Value(zz)]] => {
              //println!("OUT = {:?}", [x, y, z, zz]);
              [x, y, z, zz] == ["some", "crazy", "string", "bro"]
            },
            _ => false,
          },
          _ => false,
        },
        SplitResult::Value(_) => false,
      };
      assert_eq!(matches, true);
    }
}