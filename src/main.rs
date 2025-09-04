#[derive(Debug, PartialEq, Eq)]
// 定义几种错误情况
pub enum Error {
    InvalidInputBase,      // 输入进制不符合要求
    InvalidOutputBase,     // 输出进制不符合要求
    InvalidDigit(u32),     // 输入字串出现了大于 from_base 的字符
}

// 定义几种常见的进制前缀
const BASE2: &str = "0b";
const BASE8: &str = "0o";
const BASE16: &str = "0x";

// 任意进制之间的转换
pub fn convert(number_str: &str, from_base: u32, to_base: u32) -> Result<String, Error> {
    // 排除两种基数输入错误情况
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    };
    if to_base <= 1 || to_base > 61 {
        return Err(Error::InvalidOutputBase);
    }

    // 首先将输入的字符串转换为 十进制 数字
    let mut number = 0_i128;
    for (digit, num_char) in number_str.chars().rev().enumerate() {
        // 将 num_char 从 char 类型转换为 数值类型
        let num = match num_char {
            ('0'..='9') => (0..=9).zip('0'..='9').find(|(_, b)| *b == num_char).unwrap().0,
            ('A'..='Z') => (10..=35).zip('A'..='Z').find(|(_, b)| *b == num_char).unwrap().0,
            ('a'..='z') => (36..=61).zip('a'..='z').find(|(_, b)| *b == num_char).unwrap().0,
            _ => panic!("Invalid input!"),
        } as u32;
        if num >= from_base {
            return Err(Error::InvalidDigit(num));
        }
        number += num as i128 * (from_base as i128).pow(digit as u32);
    }

    if number == 0 {
        return Ok("0".into());
    }

    let prefix: String = match to_base {
        2 => BASE2.into(),
        8 => BASE8.into(),
        16 => BASE16.into(),
        _ => "".into(),
    };
    
	// remainder 用于保存取余的结果
    let mut remainder = Vec::new();
    while number > 0 {
        remainder.push(number % (to_base as i128));
        number /= to_base as i128;
    }
    
	// 将余数逆序
    remainder.reverse();

    let res: String = remainder.iter().fold(prefix, |mut acc, x| {
        match x {
            x @ (10..=35) => acc.push(('A'..='Z').nth((x - 10) as usize).unwrap()),
            x @ (36..=61) => acc.push(('a'..='z').nth((x - 36) as usize).unwrap()),
            _ => acc.push_str(&x.to_string()),
        }
        acc
    });

    Ok(res)
}

use clap::Parser;

#[derive(Parser)]
#[command(name = "Convert")]
#[command(author = "Pipe_2U")]
#[command(version = "0.1")]
struct Args {
    num_str: String,
    from_base: u32,
    to_base: u32,
}

fn main() {
    let args = Args::parse();
    let res = convert(&args.num_str, args.from_base, args.to_base);
    match res {
        Ok(num) => println!("{num}"),
        Err(e) => eprintln!("{e:?}"),
    }
}
