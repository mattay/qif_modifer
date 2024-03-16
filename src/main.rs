use std::{env, fs, path::Path};
use std::fs::File;
use std::io::{self, Write, BufRead, BufReader};

fn check_file(file_path: &str) -> bool{
    match fs::metadata(file_path) {
        Ok(metadata) => {
            if metadata.is_file() {
                let path = Path::new(file_path);
                return path.extension().map_or(false, |ext|ext == "qif");
            }
            metadata.is_file()
        }
        Err(_) => false
    }
}


fn clean_up_payer(raw_name: &str) -> &str {
    let prefixes_to_remove  = vec![
        "Online Purchase From",
        "Purchase From",
        "Purchase At Sq *",
        "Purchase At",
        "Payment To",
        "To",
        "From",
    ];

    let mut cleaned_name = raw_name;
    for prefix in &prefixes_to_remove {
        cleaned_name = cleaned_name.trim_start_matches(prefix)
    }

    cleaned_name.trim_start()
}

struct Transaction {
    d: String,
    p: String,
    n: String,
    m: String,
    t: String
}

fn process_qif_section(section_lines: &Vec<String>) -> Transaction{
    let mut section = Transaction {
        d: String::new(),
        p: String::new(),
        n: String::new(),
        m: String::new(),
        t: String::new(),
    };

    for line in section_lines {
        let (first_char, value) = line.split_at(1);
            if let Some(identifier) = first_char.chars().next() {
                // let value = value.to_owned();
                match identifier {
                    'D' => section.d = value.to_string(),
                    'T' => section.t = value.to_string(),
                    'N' => {if value != "" {println!("Notes: '{}'", value)}},
                    'P' => {
                        let parts: Vec<&str> = value.split(" - ").collect();
                        section.p = clean_up_payer(parts.get(0).unwrap_or(&"")).to_string();
                        section.m = parts.get(1).unwrap_or(&"").to_string();
                    },
                    _ => println!("Unknown identifier: {}", identifier),
                }
            }
    }

    section
}



fn read_and_print_qif_files(input_path: &str, output_path: &str) -> io::Result<()> {
    let file_input = File::open(input_path)?;
    let mut file_output = File::create(output_path)?;

    let reader: BufReader<File> = BufReader::new(file_input);

    let mut current_section_lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        if line.starts_with('!') {
            writeln!(file_output, "{}", line)?;

        } else if line.starts_with('^'){
            // Section end
            let section = process_qif_section(&current_section_lines);

            writeln!(file_output, "D{}", section.d)?;
            writeln!(file_output, "P{}", section.p)?;
            writeln!(file_output, "N{}", section.n)?;
            writeln!(file_output, "M{}", section.m)?;
            writeln!(file_output, "T{}", section.t)?;
            writeln!(file_output, "{}", line)?;
            // Reset lines in section
            current_section_lines.clear();

        }else{
            // Section line
            current_section_lines.push(line);
        }
    }

    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    // Exits if no arguments
    // TODO: show help
    if args.is_empty() {
        std::process::exit(0)
    }

    let input_file = &args[0];
    let output_file = &args[1];

    if check_file(&input_file) {
        if let Err(err) = read_and_print_qif_files(input_file, output_file){
            eprintln!("Error reading QIF file: {}", err);
            std::process::exit(1)
        }
    }else{
        println!("File '{}' does not exist or is not a .qif !", input_file);
    }

}
