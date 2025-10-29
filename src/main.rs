
use std::env;
use std::io::{self, Read, Write};

const BASE91_TABLE: &[u8; 91] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!#$%&()*+,./:;<=>?@[]^_`{|}~\"";
fn base91_decode_table() -> [i8; 256] {
    let mut table = [ -1i8; 256 ];
    for (i, &c) in BASE91_TABLE.iter().enumerate() {
        table[c as usize] = i as i8;
    }
    table
}

fn base91_encode(input: &[u8]) -> String {
    let mut b: u32 = 0;
    let mut n: u32 = 0;
    let mut out = Vec::new();
    for &byte in input {
        b |= (byte as u32) << n;
        n += 8;
        if n > 13 {
            let mut v = b & 8191;
            if v > 88 {
                b >>= 13;
                n -= 13;
            } else {
                v = b & 16383;
                b >>= 14;
                n -= 14;
            }
            out.push(BASE91_TABLE[(v % 91) as usize]);
            out.push(BASE91_TABLE[(v / 91) as usize]);
        }
    }
    if n > 0 {
        out.push(BASE91_TABLE[(b % 91) as usize]);
        if n > 7 || b > 90 {
            out.push(BASE91_TABLE[(b / 91) as usize]);
        }
    }
    String::from_utf8(out).unwrap()
}

fn base91_decode(input: &str, ignore_garbage: bool) -> Vec<u8> {
    let table = base91_decode_table();
    let mut b: u32 = 0;
    let mut n: u32 = 0;
    let mut v: i32 = -1;
    let mut out = Vec::new();
    for c in input.bytes() {
        let d = table[c as usize];
        if d == -1 {
            if ignore_garbage { continue; } else { break; }
        }
        if v == -1 {
            v = d as i32;
        } else {
            v += (d as i32) * 91;
            b |= (v as u32) << n;
            n += if (v & 8191) > 88 { 13 } else { 14 };
            loop {
                if n < 8 { break; }
                out.push((b & 255) as u8);
                b >>= 8;
                n -= 8;
            }
            v = -1;
        }
    }
    if v != -1 {
        b |= (v as u32) << n;
        n += 7;
        while n >= 8 {
            out.push((b & 255) as u8);
            b >>= 8;
            n -= 8;
        }
    }
    out
}

fn print_help() {
        println!("Usage: b91 [OPTION]... [FILE]\n\
Encode or decode data using base91.\n\
\nOptions:\n\
    -d, --decode           decode data\n\
    -i, --ignore-garbage   when decoding, ignore non-base91 characters\n\
    -w, --wrap=COLS        wrap encoded lines after COLS characters (default: no wrap)\n\
    -s, --split=SIZE       split output into files of SIZE bytes each (see below)\n\
    -n, --name=PREFIX      output file prefix (default: FILE.)\n\
    --digits=N             number of digits in output file suffix (default: 3)\n\
    --help                 display this help and exit\n\
\nSIZE format:\n  Integer and optional unit: K,M,G,T,P,E,Z,Y,R,Q (powers of 1024), KB,MB,... (powers of 1000),\n  or binary prefixes: KiB=K, MiB=M, etc. Examples: 10K, 5MiB, 100MB\n\
Output files: <prefix><number>.b91.txt, with <number> zero-padded to N digits.\n\
If FILE is provided, input is read from FILE. Otherwise, input is read from stdin.\n");
}

fn parse_args(args: &[String]) -> (bool, bool, Option<usize>, bool, Option<String>, Option<String>, Option<String>, usize) {
    let mut decode = false;
    let mut ignore_garbage = false;
    let mut wrap = None;
    let mut help = false;
    let mut split = None;
    let mut name = None;
    let mut file = None;
    let mut digits = 3;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-d" | "--decode" => decode = true,
            "-i" | "--ignore-garbage" => ignore_garbage = true,
            "-w" => {
                if i + 1 < args.len() {
                    wrap = args[i+1].parse().ok();
                    i += 1;
                }
            },
            s if s.starts_with("--wrap=") => {
                wrap = s[7..].parse().ok();
            },
            "-s" => {
                if i + 1 < args.len() {
                    split = Some(args[i+1].clone());
                    i += 1;
                }
            },
            s if s.starts_with("--split=") => {
                split = Some(s[8..].to_string());
            },
            "-n" => {
                if i + 1 < args.len() {
                    name = Some(args[i+1].clone());
                    i += 1;
                }
            },
            s if s.starts_with("--name=") => {
                name = Some(s[7..].to_string());
            },
            s if s.starts_with("--digits=") => {
                digits = s[9..].parse().unwrap_or(3);
            },
            "--help" | "-h" => help = true,
            _ => {
                // If not an option, treat as file argument
                if file.is_none() && !args[i].starts_with('-') {
                    file = Some(args[i].clone());
                }
            }
        }
        i += 1;
    }
    (decode, ignore_garbage, wrap, help, split, name, file, digits)
}
fn parse_size(size: &str) -> Option<usize> {
    let size = size.trim();
    if size.is_empty() { return None; }
    let mut num = String::new();
    let mut unit = String::new();
    for c in size.chars() {
        if c.is_digit(10) {
            num.push(c);
        } else {
            unit.push(c);
        }
    }
    let n: usize = num.parse().ok()?;
    let unit = unit.to_ascii_uppercase();
    let pow1024 = |exp| n.checked_mul(1024usize.pow(exp));
    let pow1000 = |exp| n.checked_mul(1000usize.pow(exp));
    match unit.as_str() {
        "" => Some(n),
        "K" | "KI" | "KIB" => pow1024(1).map(|v| v),
        "M" | "MI" | "MIB" => pow1024(2).map(|v| v),
        "G" | "GI" | "GIB" => pow1024(3).map(|v| v),
        "T" | "TI" | "TIB" => pow1024(4).map(|v| v),
        "P" | "PI" | "PIB" => pow1024(5).map(|v| v),
        "E" | "EI" | "EIB" => pow1024(6).map(|v| v),
        "Z" | "ZI" | "ZIB" => pow1024(7).map(|v| v),
        "Y" | "YI" | "YIB" => pow1024(8).map(|v| v),
        "R" | "RI" | "RIB" => pow1024(9).map(|v| v),
        "Q" | "QI" | "QIB" => pow1024(10).map(|v| v),
        "KB" => pow1000(1).map(|v| v),
        "MB" => pow1000(2).map(|v| v),
        "GB" => pow1000(3).map(|v| v),
        "TB" => pow1000(4).map(|v| v),
        "PB" => pow1000(5).map(|v| v),
        "EB" => pow1000(6).map(|v| v),
        "ZB" => pow1000(7).map(|v| v),
        "YB" => pow1000(8).map(|v| v),
        "RB" => pow1000(9).map(|v| v),
        "QB" => pow1000(10).map(|v| v),
        _ => None,
    }
}

use std::time::{Instant};

fn format_speed(bytes: f64) -> String {
    let units = ["B/s", "KB/s", "MB/s", "GB/s", "TB/s", "PB/s", "EB/s"];
    let mut val = bytes;
    let mut idx = 0;
    while val >= 1024.0 && idx < units.len() - 1 {
        val /= 1024.0;
        idx += 1;
    }
    format!("{:.2} {}", val, units[idx])
}

fn format_duration(secs: f64) -> String {
    let secs = secs.round() as u64;
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else if m > 0 {
        format!("{:02}:{:02}", m, s)
    } else {
        format!("{:02}s", s)
    }
}

fn split_output(data: &[u8], size: usize, prefix: &str, digits: usize, show_progress: bool) {
    let mut idx = 0;
    let mut file_num = 0;
    let total = data.len();
    let start = Instant::now();
    while idx < total {
        let end = std::cmp::min(idx + size, total);
        let fname = format!("{}{:0width$}.b91.txt", prefix, file_num, width=digits);
        let mut f = std::fs::File::create(&fname).expect("Failed to create output file");
        f.write_all(&data[idx..end]).expect("Failed to write to output file");
        idx = end;
        file_num += 1;
        if show_progress {
            let elapsed = start.elapsed().as_secs_f64();
            let done = idx;
            let percent = (done as f64 / total as f64) * 100.0;
            let speed = if elapsed > 0.0 { format_speed(done as f64 / elapsed) } else { "-- B/s".to_string() };
            let eta = if done > 0 {
                let remaining = total - done;
                let rate = done as f64 / elapsed;
                if rate > 0.0 {
                    format_duration(remaining as f64 / rate)
                } else {
                    "--".to_string()
                }
            } else {
                "--".to_string()
            };
            eprint!("\rProgress: {:6.2}% | ETA: {} | Speed: {}", percent, eta, speed);
        }
    }
    if show_progress {
        eprintln!("");
    }
}

fn wrap_output(s: &str, cols: usize) -> String {
    if cols == 0 { return s.to_string(); }
    let mut out = String::new();
    let mut count = 0;
    for c in s.chars() {
        out.push(c);
        count += 1;
        if count == cols {
            out.push('\n');
            count = 0;
        }
    }
    if count != 0 {
        out.push('\n');
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (decode, ignore_garbage, wrap, help, split, name, file, digits) = parse_args(&args);
    if help {
        print_help();
        return;
    }
    let mut input = Vec::new();
    let prefix = if let Some(n) = name {
        n
    } else if let Some(ref fname) = file {
        let mut p = fname.clone();
        p.push('.');
        p
    } else {
        String::from("b91")
    };
    if let Some(ref fname) = file {
        match std::fs::File::open(fname) {
            Ok(mut f) => {
                f.read_to_end(&mut input).expect("Failed to read file");
            },
            Err(e) => {
                eprintln!("Failed to open file '{}': {}", fname, e);
                std::process::exit(1);
            }
        }
    } else {
        io::stdin().read_to_end(&mut input).expect("Failed to read stdin");
    }
    let output: Vec<u8> = if decode {
        let s = String::from_utf8_lossy(&input);
        base91_decode(&s, ignore_garbage)
    } else {
        let encoded = base91_encode(&input);
        let wrapped = if let Some(cols) = wrap { wrap_output(&encoded, cols) } else { encoded };
        wrapped.into_bytes()
    };
    if let Some(size_str) = split {
        if let Some(size) = parse_size(&size_str) {
            let show_progress = file.is_some();
            split_output(&output, size, &prefix, digits, show_progress);
        } else {
            eprintln!("Invalid split size: {}", size_str);
            std::process::exit(1);
        }
    } else {
        io::stdout().write_all(&output).expect("Failed to write stdout");
    }
}
