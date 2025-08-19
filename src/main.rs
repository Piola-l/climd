use colored::*;
use regex::Regex;


fn main() {
  let italic_rex = Regex::new(r"\*(\w+)\*").unwrap();
  let bold_rex = Regex::new(r"\*\*(\w+)\*\*").unwrap();
  let strikethrough_rex = Regex::new(r"\~\~(\w+)\~\~").unwrap();
  
  let text = "*foo* **bar** ~~baz~~";

  let mut result: Vec<ColoredString> = Vec::new();

  for word in text.split_whitespace() { 
    if word.chars().nth(0).unwrap() == "#" {
      break;
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
    println!("{i}");
  }
}
