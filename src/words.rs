use regex::Regex;
use std::fs::File;
use std::io::Read;

struct Word<'a> {
  word: &'a str,
  isSafe: bool,
  isTriggered: bool
}

impl Word<'static> {
  fn trigger(&mut self) {
    self.isTriggered = true;
  }
}

pub fn parseWords() -> Vec<String> {
  let mut file = File::open("./data/el-10k.txt").expect("Open file failed!");
  let mut data = String::new();
  file.read_to_string(&mut data).expect("Read file failed!");
  println!("{}", data);
  let splitted: Vec<String> = data.split(" ").map(|s| s.to_string()).collect();
  splitted
}

pub fn checkMatch(word: &Word, regex: &str) -> bool {
  let re = Regex::new(regex).unwrap();
  let matched: bool = re.is_match(word.word);
  if matched {
    word.trigger();
  }
  !matched && word.isSafe
}
