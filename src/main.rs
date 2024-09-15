use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }
    results
}

fn main() -> Result<(), Error> {
    // Using the try `?` operator to propagate errors.
    let text: String = fs::read_to_string("logs.txt")?;
    let error_logs: Vec<&str> = extract_errors(&text);
    fs::write("errors.txt", error_logs.join("\n"))?;

    Ok(())

    // Using `expect()` to panic with a custom message on failure.
    // let text: String = fs::read_to_string("logs.txt").expect("Failed to read file logs.txt");
    // let error_logs: Vec<&str> = extract_errors(&text);

    // fs::write("errors.txt", error_logs.join("\n")).expect("Writing of errors.txt failed");

    // Using `match` for explicit error handling.
    // match fs::read_to_string("logs.txt") {
    //     Ok(log) => {
    //         let error_logs = extract_errors(&log);

    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("Successfully wrote to file"),
    //             Err(error) => println!("Writing of errors.txt failed: {}", error),
    //         }
    //     }

    //     Err(err) => println!("Failed to read file: {}", err),
    // }
}
