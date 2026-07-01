use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant, SystemTime};

pub enum Value{
    Int(i32),
    Str(String),
    List(Vec<Value>),
}

pub enum Command{
    Get(String),
    Set(String, (Value, Instant)),
    Del(String),
    Unknown,
}

pub fn init_memory() -> HashMap<String, (Value, Instant)> {
    HashMap::new()
}

pub fn run_command(line: &str, hash_map: &mut HashMap<String, (Value, Instant)>) {
    let now = Instant::now();
    let command = parse_line(line);
    match command {
        Command::Get(key) => {
            match hash_map.get(&key) {
                Some((value, expire)) => {
                    if now <= *expire {
                        println!("Value for key '{key}': {value}. Expires at {expire:?}")
                    } else {
                        println!("No value found for key '{key}");
                        hash_map.remove(&key);
                    }
                },
                None => println!("No value found for key '{key}'"),
            } 
        },
        Command::Set(key, (value, expire)) => {hash_map.insert(key, (value, expire));},
        Command::Del(key) => {hash_map.remove(&key);},
        Command::Unknown => println!("Unknown command"),
    }
}

pub fn parse_line(line: &str) -> Command {
    let now = Instant::now();
    let words: Vec<&str> = line.split_whitespace().collect();
    let expire_index = words.iter().position(|word| *word == "EX").unwrap_or(words.len());
    let expire_value = line[expire_index..].trim_start_matches("EX").trim().parse::<i32>().unwrap_or(1);
    let expire_time = now.checked_add(Duration::from_secs(expire_value as u64)).unwrap();

    match words.first().copied() {
        Some("GET") =>  Command::Get(String::from(words[1..expire_index].join(" ").to_string())),
        Some("SET") => {
            let key = String::from(words[1]);
            let value = Value::Str(words[2..expire_index].join(" ").to_string());
            Command::Set(key, (value, expire_time))
        },
        Some("DEL") => Command::Del(String::from(words[1])),
        _ => Command::Unknown,
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Str(s) => write!(f, "{s}"),
            Value::List(items) => {
                let rendered_items: Vec<String> = items
                    .iter()
                    .map(|item| item.to_string())
                    .collect();

                write!(f, "[{}]", rendered_items.join(", "))
            }
        }
    }
}