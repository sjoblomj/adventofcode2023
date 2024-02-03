use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let file = File::open("src/input1.txt").expect("Unable to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    //println!("{}", contents);
//    part("one", &contents, |x| x.to_string()); // 55002
//    part("two", &contents, words_to_nums); // > 54857
    //part("one", &contents); // 55002
    part("two", &words_to_nums(&contents)); // > 55061
    Ok(())
}

//fn part(part: &str, contents: &str, line_modifyer: fn(&str) -> String) {
fn part(part: &str, contents: &str) {
    let mut sum = 0;
    for line in contents.lines() {
        //let l = line_modifyer(line);
//        let l = words_to_nums(line);
        //let l = line;
        //println!("{line}  =>  {l}");
        //println!("{line}");
        let mut fst = '\0';
        let mut snd = '\0';
        for c in line.chars() {
            if fst == '\0' && c.is_numeric() {
                fst = c;
            }
            if c.is_numeric() {
                snd = c;
            }
        }
        //println!("{} {}", fst, snd);
        let n = (10 * fst.to_digit(10).unwrap()) + snd.to_digit(10).unwrap();
        sum += n;
        //let s = fst.concat(snd);
        //let n = s.parse::<i32>().unwrap();
        //println!("{} {} => {}", fst, snd, n);
    }
    println!("Part {part}: The sum of the calibration values: {sum}");
/*
    let re = Regex::new(r"^.*(?<fst>\d).*(?<snd>\d)?.*$").unwrap();
    contents
        .lines()
        .for_each(|l| {
            match re.captures(l) {
                Some(ns) => {
                    let fst = ns.get(1).map_or("", |n| n.as_str());
                    let snd = ns.get(2).map_or(fst, |n| n.as_str());
                    println!("{} {}", fst, snd);
                }
                None => {
                    println!("NOES");
                    
                }
            }
//            let ns = re.captures(l).unwrap();
            //let fst = &ns["fst"];
//            let snd = ns.get(2).map_or(fst, |n| n.as_str());
        });
*/
}

fn words_to_nums(string: &str) -> String {
    let map = HashMap::from([
        ("one",   "1"),
        ("two",   "2"),
        ("three", "3"),
        ("four",  "4"),
        ("five",  "5"),
        ("six",   "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine",  "9"),
    ]);
    let mut s: String = string.into();
    loop {
        let find_tuples: Vec<_> = map.keys()
            .map(|k| (k, s.find(k)))
            .filter(|pair| pair.1.is_some())
            .collect();

        if let Some(tup) = find_tuples.iter().min_by_key(|tuple| tuple.1) {
        //println!("{:?}, choosing {:?}", find_tuples, tup);
            s = s.replacen(tup.0, map[tup.0], 1);
        } else {
            break;
        }
    }
    return s;
}
