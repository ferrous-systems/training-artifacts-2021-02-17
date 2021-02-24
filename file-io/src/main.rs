



use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use url::Url;

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

type IoResult<T> = Result<T, std::io::Error>;

fn foo() -> Result<(), ()> {
    // Ok(()) // Good!
    Err(()) // Good!
}

fn main() -> std::io::Result<()> {
    let f = File::open("src/data/content.txt");

    // match the two possible patterns,
    // Ok(file) and Err(e) to an an appropriate
    // expression, for example:
    // println!("File opened") and
    // println!("Error: {}", e)


    let mut file = match f {
        // substitute this placeholder for the two possible patterns from the result type
        Ok(f) => {
            println!("File opened!");
            f
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    let mut reader = BufReader::new(file);

    let mut lines_count = 0;
    let mut sig_line_count = 0;

    // Optional: Read ALL file contents into RAM
    let mut all_lines: Vec<String> = Vec::new();

    // "Code Golf" approach
    //
    // let all_lines = reader
    //     .lines()
    //     .filter_map(Result::ok)
    //     .filter(|l| !l.is_empty())
    //     .collect::<Vec<String>>();

    for line in reader.lines() {
        if let Ok(line) = line {
            if !line.is_empty() {
                all_lines.push(line);
            }
        }
    }

    let line_len = all_lines.len();

    for line in all_lines {
        if let Some(url_line) = parse_url(line) {
            println!("{}", url_line);
        } else {
            return Err(/* ... */);
        }
    }

    println!("Total lines: {}", line_len);

    Ok(())
}

// enum Option<T> {
//     Some(T),
//     None
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn parse_url(line: String) -> Option<Url> {
    match Url::parse(&line) {
        Ok(url) => Some(url),
        Err(e) => {
            // Maybe print?
            None
        }
    }
}

// fn unwrap(result: Result<T, E>) -> T {
//     match result {
//         Ok(t) => return t,
//         Err(e) => {
//             panic!("{}", e)
//         }
//     }
// }
