use std::{env, fs, collections::HashMap};
use colored::*;
use regex::Regex;

fn build_regexmap() -> HashMap<String, Regex> {
  let mut map: HashMap<String, Regex> = HashMap::new();
  
  let bold_rex = Regex::new(r"\*\*(\w+)\*\*").unwrap();
  let italic_rex = Regex::new(r"(?:^|[^*])\*([^*]+)\*(?:[^*]|$)").unwrap();
  let strikethrough_rex = Regex::new(r"\~\~(.+?)\~\~").unwrap();

  map.insert("italic".to_string(), italic_rex); 
  map.insert("bold".to_string(), bold_rex); 
  map.insert("strikethrough".to_string(), strikethrough_rex);

  map
}


fn count_captured<'a>(map: &'a HashMap<String, Regex>, string: &'a String) -> String {
  let mut result = string.to_owned();

  for (k,v) in map {
    if let Some(cap) = v.captures(string) {
      println!("{} for {:?}", k, string);
      result = style_by_string(&cap[1].to_string(), &k).to_string();
      //captured.push(k.to_owned());
    }
  }

  result
}

fn style_by_string(text: &String, style: &String) -> ColoredString {
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

  let mut result: Vec<String> = Vec::new();
 
  for line in text.split_inclusive('\n') {
    let mut iter = line.split(' ');
   
    while let Some(word) = iter.next() {
      let mut word_final: String = word.to_string();

      //println!("{:?} for {}", 
      //  count_captured(&rexmap, word.to_string());
      //  word
      //  );

      if word == "\n" {
        result.push("\n".normal().to_string());
        continue;
      }

      // Header 1
      //if word_final.starts_with("#") {
        //let header = iter.clone().collect::<Vec<&str>>().join(" ");
        //word_final = header.reversed().to_string();
        //result.push(word_final.clone());
        //break; // остальная строка уже обработана
      //}

      println!("w: {}", word);
      let word = count_captured(&rexmap, &word_final.to_string());
      word_final = word;
      println!("w f: {} \n", word_final);

      result.push(word_final.clone());
      result.push(" ".to_string());
    }
  }

  for i in result {
    print!("{i}");
  }

  println!();

}
