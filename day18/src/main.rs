use std::fs;
use regex::Regex;

//part 1
// fn calc_line (line: String) -> String {
//     let mut result = line;

//     let fin_re = Regex::new(r"^\d+$").unwrap();
//     if fin_re.is_match(&result) {
//         return result;
//     }

//     let par_re = Regex::new(r"\(([\d|\*|\+]+)\)").unwrap();
//     if par_re.is_match(&result) {
//         let caps = par_re.captures(&result).unwrap();

//         let par_result = calc_line(String::from(caps.get(1).map_or("", |m| m.as_str())));

//         result = String::from(par_re.replace(&result, &par_result[..]))
//     }
//     let oper_re = Regex::new(r"^(\d+)(\*|\+)(\d+)").unwrap();
//     if oper_re.is_match(&result) {
//         let caps = oper_re.captures(&result).unwrap();
//         let left = caps.get(1).map_or(0, |m| m.as_str().parse::<i64>().unwrap());
//         let right = caps.get(3).map_or(0, |m| m.as_str().parse::<i64>().unwrap());
//         let oper = caps.get(2).map_or("", |m| m.as_str());
//         let oper_res = if oper == "*" {left*right} else {left+right};
//         result = String::from(oper_re.replace(&result, &oper_res.to_string()[..]));
//     }

//     calc_line(result)
// }


/////////////////////////////////////////////////

/// part 2

fn calc_line (line: String) -> String {
    let mut result = line;

    let fin_re = Regex::new(r"^\d+$").unwrap();
    if fin_re.is_match(&result) {
        return result;
    }

    let par_re = Regex::new(r"\(([\d|\*|\+]+)\)").unwrap();
    if par_re.is_match(&result) {
        let caps = par_re.captures(&result).unwrap();

        let par_result = calc_line(String::from(caps.get(1).map_or("", |m| m.as_str())));

        result = String::from(par_re.replace(&result, &par_result[..]))
    }
    let plus_re = Regex::new(r"(\d+)(\+)(\d+)").unwrap();
    if plus_re.is_match(&result) {
        let caps = plus_re.captures(&result).unwrap();
        let left = caps.get(1).map_or(0, |m| m.as_str().parse::<i64>().unwrap());
        let right = caps.get(3).map_or(0, |m| m.as_str().parse::<i64>().unwrap());
        let plus_res = left + right;
        result = String::from(plus_re.replace(&result, &plus_res.to_string()[..]));
    }

    let mult_re = Regex::new(r"(\d+)(\*)(\d+)").unwrap();
    if !result.contains("+") && mult_re.is_match(&result) {
        let caps = mult_re.captures(&result).unwrap();
        let left = caps.get(1).map_or(0, |m| m.as_str().parse::<i64>().unwrap());
        let right = caps.get(3).map_or(0, |m| m.as_str().parse::<i64>().unwrap());
        let mult_res = left * right;
        result = String::from(mult_re.replace(&result, &mult_res.to_string()[..]));
    }

    calc_line(result)
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();
    let mut res = 0;
    for line in lines {
        let re = Regex::new(r"\s").unwrap();
        let no_spaces = String::from(re.replace_all(&line, ""));
        res += calc_line(no_spaces).parse::<i64>().unwrap();
    }
    println!("{}", res);
}
