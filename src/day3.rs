
pub fn day3() -> std::io::Result<()> {
    let content = std::fs::read_to_string("inputs/input3.txt")
        .expect("Unable to open file");

    println!("Part 1: Sum of active parts: {}", calculate_sum_of_active_parts(&content)); // > 526389 && < 582658 && != 534337
    println!("Part 2: Sum of powers: powers");

    Ok(())
}

fn calculate_sum_of_active_parts(lines: &str) -> u32 {
    let (nums, syms) = parse_engine_schematic(lines);
    syms.iter().for_each(|s| println!(" {}", s.to_string()));
    return nums.iter()
        .filter(|n| if syms.iter().any(|s| s.has_overlap(n)) { true } else { println!("{}", n.to_string()); false})
        .map(|n| n.value.parse::<u32>().unwrap())
        .sum();
}

fn parse_engine_schematic(lines: &str) -> (Vec<RectangleValue>, Vec<RectangleValue>) {
    let mut i = 0;
    let mut nums: Vec<RectangleValue> = Vec::new();
    let mut syms: Vec<RectangleValue> = Vec::new();

    for l in lines.lines() {
        let (ns, ss) = parse_line(i, l);
        for n in ns {
            nums.push(n);
        }
        for s in ss {
            syms.push(s);
        }
        i += 1;
    }
    return (nums, syms);
}

fn parse_line(line_num: u32, line: &str) -> (Vec<RectangleValue>, Vec<RectangleValue>) {
    let mut nums: Vec<RectangleValue> = Vec::new();
    let mut syms: Vec<RectangleValue> = Vec::new();

    let mut num = String::new();
    let mut index: u32 = 0;
    for c in line.chars() {
        if c.is_numeric() {
            num.push(c);
        } else if !num.is_empty() {
            let num_len: u32 = num.chars().count().try_into().unwrap();
            let y: u32 = if line_num == 0 { 0 } else { line_num - 1 };
            nums.push(RectangleValue{
                value: num.clone(),
                x: if num_len + 1 > index { 0 } else { index - num_len - 1 },
                y,
                width: (num.chars().count() + 2).try_into().unwrap(),
                height: if y == 0 { 2 } else { 3 },
            });
            num = String::new();
        }

        if !c.is_numeric() && c != '.' {
            syms.push(RectangleValue{
                value: c.to_string(),
                x: index,
                y: line_num,
                width: 0,
                height: 0,
            });
        }
        index += 1;
    }

    return (nums, syms);
}

struct RectangleValue {
    value: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl RectangleValue {
    fn has_overlap(&self, othr: &RectangleValue) -> bool {
        let ((alx, aly), (arx, ary)) = ((self.x, self.y), (self.x + self.width, self.y + self.height));
        let ((blx, bly), (brx, bry)) = ((othr.x, othr.y), (othr.x + othr.width, othr.y + othr.height));
        return if alx <= brx && arx >= blx && aly <= bry && ary >= bly {
            true
        } else {
            /*
            println!("{}", self.to_string());
            println!("{}", othr.to_string());
            println!("{} && {} && {} && {}", alx <= brx , arx >= blx , aly <= bry , ary >= bly);
             */
            false
        }
    }

    fn to_string(&self) -> String {
        return format!("[v {}, ({}, {}), ({}, {})]", self.value, self.x, self.y, self.x + self.width, self.y + self.height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_sum_of_active_parts() {
        let content = std::fs::read_to_string("inputs/input3_test.txt")
            .expect("Unable to open file");
        assert_eq!(4361, calculate_sum_of_active_parts(&content));
    }

    #[test]
    fn can_parse_lines() {
        let content = std::fs::read_to_string("inputs/input3_test.txt")
            .expect("Unable to open file");
        let (nums, syms) = parse_engine_schematic(&content);
        assert_eq!(10, nums.len());
        assert_eq!(6, syms.len());
    }

    #[test]
    fn can_parse_single_line() {
        let (nums0, syms0) = parse_line(0, "..........");
        assert_eq!(0, nums0.len());
        assert_eq!(0, syms0.len());

        let (nums1, syms1) = parse_line(0, ".....+.58.");
        assert_eq!(1, nums1.len());
        assert_eq!(1, syms1.len());

        let (nums2, syms2) = parse_line(0, "...........749....*...811..........*667......257.+309.477-.*..............212......746.........275*.............264......*..........720.....");
        assert_eq!(11, nums2.len());
        assert_eq!(7, syms2.len());
    }

    #[test]
    fn can_create_rectangles_and_see_if_contained() {
        let r0 = RectangleValue {
            value: "apa".to_string(),
            x: 1,
            y: 1,
            width: 3,
            height: 4,
        };
        let r1 = RectangleValue {
            value: "bepa".to_string(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        let r2 = RectangleValue {
            value: "cepa".to_string(),
            x: 2,
            y: 2,
            width: 2,
            height: 2,
        };
        let r3 = RectangleValue {
            value: "*".to_string(),
            x: 3,
            y: 1,
            width: 0,
            height: 0,
        };
        let r4 = RectangleValue {
            value: "467".to_string(),
            x: 0,
            y: 0,
            width: 5,
            height: 1,
        };

        assert!(!r0.has_overlap(&r1));
        assert!(r0.has_overlap(&r2));
        assert!(r4.has_overlap(&r3));
    }
}
