use std::io::{self,BufRead};
use std::ops::Range;
use std::str;
use failure::{Error,format_err};
use clap::clap_app;

fn main() -> Result<(), Error>{
  let matches = clap_app!(bisect =>
                          (version: "0.2")
                          (author: "Judson Lester")
                          (about: "bisection search of stdin lines")
                          (@arg BITSTRING: +required "list of 0,1 to define bisection path")
                          (@arg invert: -i --invert "supress selected lines instead of printing them")
                         ).get_matches();

  let stdin = io::stdin();
  let handle = stdin.lock();
  let lines = handle.lines().collect::<Result<Vec<_>,_>>().map_err(|e| Error::from(e))?;

  let range = range(lines.len(), matches.value_of("BITSTRING").unwrap())?;

  if matches.is_present("invert") {
          let (before, after) = range_diff(0..lines.len(), range);
          print_range(lines.clone(), before);
          print_range(lines, after)
  } else {
      print_range(lines, range)
  }
  Ok(())
}

fn print_range(lines: Vec<String>, range: Range<usize>) {
  for l in lines.get(range).unwrap() {
      println!("{}", l)
  }
}

fn range(sz: usize, bitstring: &str) -> Result<Range<usize>, Error> {
  Ok(bisected_range(0..sz, bools(bitstring)?.into_iter()))
}

fn bisected_range(full: Range<usize>, path: impl Iterator<Item=bool>) -> Range<usize> {
    path.fold(full, |r, b| {
        let mid = (r.start + r.end) / 2;
        if b {
            (mid..r.end)
        } else {
            (r.start..mid)
        }
    })
}

fn range_diff(big: Range<usize>, little: Range<usize>) -> (Range<usize>, Range<usize>) {
    (
        Range{start: big.start, end: little.start},
        Range{start: little.end, end: big.end}
    )
}

fn bools<'g>(bitstring: &str) -> Result<Vec<bool>, Error> {
  bitstring.bytes().map(|bit|
    match bit {
      b'0' => Ok(false),
      b'1' => Ok(true),
      no => Err(format_err!("Expected a string of 0 and 1, got a {}", str::from_utf8(&[no]).unwrap()))
    }
  ).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bool_parses() {
        assert_eq!(bools("01011").unwrap(), vec![false, true, false, true, true])
    }

    #[test]
    #[should_panic]
    fn bool_errors() {
        bools("0101x").unwrap();
    }

    #[test]
    fn bisects_ranges() {
        assert_eq!(bisected_range(0..100,  vec![false,  false,  false,  false].into_iter()),  0..6);
        assert_eq!(bisected_range(0..100,  vec![false,  false,  false,  true].into_iter()),   6..12);
        assert_eq!(bisected_range(0..100,  vec![false,  false,  true,   false].into_iter()),  12..18);
        assert_eq!(bisected_range(0..100,  vec![false,  false,  true,   true].into_iter()),   18..25);
        assert_eq!(bisected_range(0..100,  vec![false,  true,   false,  false].into_iter()),  25..31);
        assert_eq!(bisected_range(0..100,  vec![false,  true,   false,  true].into_iter()),   31..37);
        assert_eq!(bisected_range(0..100,  vec![false,  true,   true,   false].into_iter()),  37..43);
        assert_eq!(bisected_range(0..100,  vec![false,  true,   true,   true].into_iter()),   43..50);
        assert_eq!(bisected_range(0..100,  vec![true,   false,  false,  false].into_iter()),  50..56);
        assert_eq!(bisected_range(0..100,  vec![true,   false,  false,  true].into_iter()),   56..62);
        assert_eq!(bisected_range(0..100,  vec![true,   false,  true,   false].into_iter()),  62..68);
        assert_eq!(bisected_range(0..100,  vec![true,   false,  true,   true].into_iter()),   68..75);
        assert_eq!(bisected_range(0..100,  vec![true,   true,   false,  false].into_iter()),  75..81);
        assert_eq!(bisected_range(0..100,  vec![true,   true,   false,  true].into_iter()),   81..87);
        assert_eq!(bisected_range(0..100,  vec![true,   true,   true,   false].into_iter()),  87..93);
        assert_eq!(bisected_range(0..100,  vec![true,   true,   true,   true].into_iter()),   93..100);
    }
}
