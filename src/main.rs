use std::{env, fs, collections::HashMap};
use colored::*;
use regex::Regex;


fn build_regexmap() -> HashMap<String, Regex> {
  let mut map: HashMap<String, Regex> = HashMap::new();
  
  let italic_rex = Regex::new(r"\*(\w+)\*").unwrap();
  let bold_rex = Regex::new(r"\*\*(\w+)\*\*").unwrap();
  let strikethrough_rex = Regex::new(r"~~(.+?)~~").unwrap();

  map.insert("italic".to_string(), italic_rex); 
  map.insert("bold".to_string(), bold_rex); 
  map.insert("strikethrough".to_string(), strikethrough_rex);

  map
}


fn count_captured<'a>(map: &'a HashMap<String, Regex>, string: &'a str) -> Vec<String> {
  let mut captured: Vec<String> = Vec::new();

  for (k,v) in map {
    if let Some(_) = v.captures(string) {
      captured.push(k.to_owned());
    }
  }

  captured
}

fn style_by_string(text: ColoredString, style: String) -> ColoredString {
  match style.as_str() {
    "italic" => text.italic(),
    "bold" => text.bold(),
    "strikethrough" => text.strikethrough(),
    _ => text.normal()
  }
}

fn main() {
  let rexmap = build_regexmap(); 

  let args: Vec<String> = env::args().collect();
  let text = fs::read_to_string(&args[1]).expect("Error reading the file.");

  let mut result: Vec<ColoredString> = Vec::new();
 
  for line in text.split_inclusive('\n') {
    let mut iter = line.split(' ').peekable();
   
    while let Some(word) = iter.next() {
      let mut word_final: ColoredString = word.normal();

      println!("{:?} for {}", 
        count_captured(&rexmap, word.to_string().as_str()),
        word
        );

      for _ in 0..3 {
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

        for style in count_captured(&rexmap, word.to_string().as_str()) {
          word_final = style_by_string(word_final, style);
        }
      }

      result.push(word_final.clone());
      result.push(" ".normal());
    }
  }

  for i in result {
    print!("{i}");
  }

  println!();

}
