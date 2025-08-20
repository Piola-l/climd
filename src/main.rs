use colored::*;
use regex::Regex;


fn main() {
  let italic_rex = Regex::new(r"\*(\w+)\*").unwrap();
  let bold_rex = Regex::new(r"\*\*(\w+)\*\*").unwrap();
  let strikethrough_rex = Regex::new(r"\~\~(\w+)\~\~").unwrap();
  
  let text = "
    # Header 1 \n
    *foo* **bar** ~~baz~~";

  let mut iter = text.split_whitespace().peekable();
  let mut result: Vec<ColoredString> = Vec::new();
 
  while let Some(word) = iter.next() {
    if word.starts_with("#") {
      if let Some(next_word) = iter.next() {
        result.push(next_word.reversed());
      }
    }

    // bold
    if let Some(cap) = bold_rex.captures(word) {
      result.push(cap[1].bold());
    }
    
    // italic
    else if let Some(cap) = italic_rex.captures(word) {
      result.push(cap[1].italic());
    }

    // strikethrough
    else if let Some(cap) = strikethrough_rex.captures(word) {
      result.push(cap[1].strikethrough());
    }

    else {
      result.push(word.normal());
    }

  }
 
  //result.push("normal".normal());
  //result.push("purple".purple());
  //result.push("bold".bold());
  //result.push("italic".italic());
  //result.push("strikethrough".strikethrough());
  //result.push("reversed".reversed());

  for i in result {
    print!("{i}");
  }
}
