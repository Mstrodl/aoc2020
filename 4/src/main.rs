use std::fs;

fn main() {
  let input = fs::read_to_string("./input.txt").unwrap();
  // let input = "byr:2001 iyr:2010 eyr:2020 hgt:150cm hcl:#0 ecl:nah pid:1";

  let mut count = 0;
  'passportLoop: for passport in input.split("\n\n") {
    for item in ["byr:","iyr:","eyr:","hgt:","hcl:","ecl:","pid:"].iter() {
      if !passport.contains(item) {
        continue 'passportLoop;
      }
    }
    // Part 2:
    let chars: Vec<char> = passport.chars().collect();
    let mut state = 0;
    let mut key = String::new();
    let mut value = String::new();
    // let mut values: HashMap<String, String> = HashMap::new();
    for char in chars {
      if char == '\n' || char == ' ' {
        if key.len() != 0 && value.len() != 0 {
          println!("Validating {}: {}", key, value);
          // values.insert(key, value);
          if key == "byr" {
            let year = u32::from_str_radix(&value, 10);
            if year.is_err() {
              println!("byr is invalid: {}", value);
              continue 'passportLoop;
            } else {
              println!("Year is a good number");
              let yearNum = year.unwrap();
              if yearNum < 1920 || yearNum > 2002 {
                continue 'passportLoop;
              }
              println!("year num is okay {}", yearNum);
            }
          } else if key == "iyr" {
            let year = u32::from_str_radix(&value, 10);
            if year.is_err() {
              println!("iyr is invalid: {}", value);
              continue 'passportLoop;
            } else {
              let yearNum = year.unwrap();
              if yearNum < 2010 || yearNum > 2020 {
                continue 'passportLoop;
              }
            }
          } else if key == "eyr" {
            let year = u32::from_str_radix(&value, 10);
            if year.is_err() {
              println!("eyr is invalid: {}", value);
              continue 'passportLoop;
            } else {
              let yearNum = year.unwrap();
              if yearNum < 2020 || yearNum > 2030 {
                continue 'passportLoop;
              }
            }
          } else if key == "hgt" {
            let mut heightOpt = value.strip_suffix("cm");
            if heightOpt.is_none() {
              heightOpt = value.strip_suffix("in");
              if heightOpt.is_none() {
                continue 'passportLoop;
              }
              let heightStr = heightOpt.unwrap();
              let heightNumRes = i32::from_str_radix(&heightStr, 10);
              if heightNumRes.is_err() {
                continue 'passportLoop;
              }
              let heightNum = heightNumRes.unwrap();
              if heightNum < 59 || heightNum > 76 {
                continue 'passportLoop;
              }
            } else {
              let heightStr = heightOpt.unwrap();
              let heightNumRes = i32::from_str_radix(&heightStr, 10);
              if heightNumRes.is_err() {
                continue 'passportLoop;
              }
              let heightNum = heightNumRes.unwrap();
              if heightNum < 150 || heightNum > 193 {
                continue 'passportLoop;
              }
            }
          } else if key == "hcl" {
            if value.len() != 7 {
              continue 'passportLoop;
            }
            let colorOpt = value.strip_prefix("#");
            if colorOpt.is_none() {
              continue 'passportLoop;
            }
            let color = colorOpt.unwrap();
            if u32::from_str_radix(&color, 16).is_err() {
              continue 'passportLoop;
            }
          } else if key == "ecl" {
            let mut failed = true;
            for color in [String::from("amb"), String::from("blu"), String::from("brn"), String::from("gry"), String::from("grn"), String::from("hzl"), String::from("oth")].iter() {
              if value == *color {
                failed = false;
                break;
              }
            }
            if failed {
              continue 'passportLoop;
            }
          } else if key == "pid" {
            if value.len() != 9 || u32::from_str_radix(&value, 10).is_err() {
              continue 'passportLoop;
            }
          } else if key != "cid" {
            panic!("Unknown key: {} = {}", key, value);
          }
          println!("{} = {} is valid", key, value);
          value = String::new();
        }

        key = String::new();
        state = 0;
      } else if char == ':' && state == 0 {
        state = 1;
      } else {
        if state == 0 {
          key.push(char);
        } else {
          value.push(char);
        }
      }
    }

    count += 1;
  }

  println!("Passports: {}", count);
}
