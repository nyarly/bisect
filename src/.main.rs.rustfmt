use std::io::{self,BufRead};
use std::env::args;
use std::ops::Range;
use std::str;

fn main() -> Result<(), String>{
  let args = args().collect::<Vec<_>>();

  if args.len() != 2 {
    return Err(String::from("Expected exactly one argument!"))
  }

  let stdin = io::stdin();
  let handle = stdin.lock();
  let lines = handle.lines().collect::<Vec<_>>();

  for r in ranges(lines.len(), args[1].clone())? {
    for l in lines.get(r).unwrap() {
      match l {
        Ok(s) => println!("{}", s),
        _ => panic!("borked")
      }
    }
  }

  Ok(())
}

fn ranges(sz: usize, bitstring: String) -> Result<Vec<Range<usize>>, String> {
  let (v, _) = bools(bitstring)?.iter().fold((vec![], 0..sz), |(mut v, r), b| {
    let (top, bottom) = range_halves(r);
    if *b {
      v.push(bottom);
      (v, top)
    } else {
      v.push(top);
      (v, bottom)
    }
  });

  Ok(v)
}

fn range_halves(full: Range<usize>) -> (Range<usize>, Range<usize>) {
  let mid = (full.start + full.end) / 2;
  ((full.start..mid), (mid..full.end))
}

fn bools(bitstring: String) -> Result<Vec<bool>, String> {
  bitstring.bytes().map(|bit|
    match bit {
      b'0' => Ok(false),
      b'1' => Ok(true),
      no => Err(format!("Expected a string of 0 and 1, got a {}", str::from_utf8(&[no]).unwrap()))
    }
  ).collect()
}
