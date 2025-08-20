use std::collections::HashMap;

use colored::*;
use regex::Regex;

fn count_captured(map: &HashMap<&str, Regex>, string: &str) -> u32 {
  let mut captured_count = 0;
  for i in map.values() {
    if let Some(_) = i.captures(string) {
      captured_count += 1;
    }
  }

  captured_count
}

fn main() {
  let mut rexmap: HashMap<&str, Regex> = HashMap::new();
  let italic_rex = Regex::new(r"\*(\w+)\*").unwrap();
  let bold_rex = Regex::new(r"\*\*(\w+)\*\*").unwrap();
  let strikethrough_rex = Regex::new(r"~~(.+?)~~").unwrap();
  
  rexmap.insert("italic", Regex::new(r"\*(\w+)\*").unwrap());
  rexmap.insert("bold", Regex::new(r"\*\*(\w+)\*\*").unwrap());
  rexmap.insert("strikethrough", Regex::new(r"~~(.+?)~~").unwrap());

  let text = 
"
# Header 1 
*foo* ~~**bar**~~ ~~baz~~
"
  ;

  let mut result: Vec<ColoredString> = Vec::new();
 
  for line in text.split_inclusive('\n') {
    let mut iter = line.split(' ').peekable();
   
    while let Some(word) = iter.next() {
      let mut word_final: ColoredString = word.normal();

      println!("{} for {}", 
        count_captured(&rexmap, word.to_string().as_str()),
        word
        );

      for _ in 0..count_captured(&rexmap, word.to_string().as_str()) {
        if word == "\n" {
          result.push("\n".normal());
          continue;
        }

        // Header 1
        if word_final.starts_with("#") {
          let header = iter.clone().collect::<Vec<&str>>().join(" ");
          word_final = header.reversed();
          result.push(word_final.clone());
          break; // остальная строка уже обработана
        }

        // bold
        if let Some(cap) = bold_rex.captures(word_final.to_string().as_str()) {
          println!("boldcap1: {}", &cap[1]);
          word_final = cap[1].bold();
        }

        // italic
        if let Some(cap) = italic_rex.captures(word_final.to_string().as_str()) {
          word_final = cap[1].italic();
        }

        // strikethrough
        if let Some(cap) = strikethrough_rex.captures(word) {
          word_final = cap[1].strikethrough();
        }

      }

      result.push(word_final.clone());
      result.push(" ".normal());
    }
  }

  //result.push("normal".normal());
  //result.push("purple".purple());
  //result.push("bold".bold());
  //result.push("italic".italic());
  //result.push("strikethrough".strikethrough().bold);
  //result.push("reversed".reversed());

  for i in result {
    print!("{i}");
  }
}
