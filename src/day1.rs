use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn day1() -> std::io::Result<()> {
    let file = File::open("src/input1.txt").expect("Unable to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    part("one", &contents); // 55002
    Ok(())
}

fn part(part: &str, contents: &str) {
    let mut sum = 0;
    for line in contents.lines() {
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
        let n = (10 * fst.to_digit(10).unwrap()) + snd.to_digit(10).unwrap();
        sum += n;
    }
    println!("Part {part}: The sum of the calibration values: {sum}");
}
