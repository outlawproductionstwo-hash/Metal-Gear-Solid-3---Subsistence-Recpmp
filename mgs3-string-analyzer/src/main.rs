use goblin::elf::Elf;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

/// Extract ASCII strings (printable characters) from a byte slice
fn extract_ascii_strings(data: &[u8], min_length: usize) -> Vec<String> {
    let mut strings = Vec::new();
    let mut current = String::new();
    
    for &byte in data {
        // Check if byte is printable ASCII (32-126) or common whitespace
        if byte >= 32 && byte <= 126 {
            current.push(byte as char);
        } else if byte == b'\n' || byte == b'\r' || byte == b'\t' {
            // Treat whitespace as string terminators for cleaner output
            if current.len() >= min_length {
                strings.push(current);
            }
            current = String::new();
        } else {
            // Non-printable character ends current string
            if current.len() >= min_length {
                strings.push(current);
            }
            current = String::new();
        }
    }
    
    // Don't forget the last string
    if current.len() >= min_length {
        strings.push(current);
    }
    
    strings
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <elf_file> [min_length] [output_file]", args[0]);
        eprintln!("Example: {} Extracted ELFs/SLUS_213.59 4 strings.txt", args[0]);
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    let min_length = if args.len() > 2 {
        args[2].parse().unwrap_or(4)
    } else {
        4
    };
    let output_path = if args.len() > 3 {
        &args[3]
    } else {
        "strings_output.txt"
    };
    
    println!("Analyzing {} for strings (min length: {})", file_path, min_length);
    println!("Output will be saved to: {}", output_path);
    
    // Read the file
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    println!("File size: {} bytes", buffer.len());
    
    // Try to parse as ELF to get section information
    match Elf::parse(&buffer) {
        Ok(elf) => {
            println!("Successfully parsed as ELF file");
            println!("Entry point: 0x{:x}", elf.entry);
            
            // Find .text section by looking for PROGBITS section
            let mut text_section = None;
            for section in &elf.section_headers {
                if section.sh_type == goblin::elf::section_header::SHT_PROGBITS {
                    text_section = Some(section);
                    break;
                }
            }
            
            let mut all_strings = Vec::new();
            
            if let Some(header) = text_section {
                let start = header.sh_offset as usize;
                let end = (header.sh_offset + header.sh_size) as usize;
                if end <= buffer.len() {
                    let text_section_data = &buffer[start..end];
                    println!("\n=== Extracting strings from .text section ({} bytes) ===", text_section_data.len());
                    
                    let ascii_strings = extract_ascii_strings(text_section_data, min_length);
                    all_strings.extend(ascii_strings.into_iter().map(|s| format!("[.text] {}", s)));
                }
            }
            
            // Also scan the whole file but label them appropriately
            println!("\n=== Extracting strings from entire file ===");
            let ascii_strings = extract_ascii_strings(&buffer, min_length);
            all_strings.extend(ascii_strings.into_iter().map(|s| format!("[file] {}", s)));
            
            // Write to output file
            let mut output_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(output_path)?;
                
            for string in &all_strings {
                writeln!(output_file, "{}", string)?;
            }
            
            println!("\n=== Summary ===");
            println!("Total strings found: {}", all_strings.len());
            println!("Results saved to: {}", output_path);
            
            // Show first 20 strings as preview
            println!("\n=== Preview (first 20 strings) ===");
            for string in all_strings.iter().take(20) {
                println!("{}", string);
            }
        }
        Err(e) => {
            println!("Failed to parse as ELF ({}), treating as raw binary", e);
            println!("Scanning entire file for strings...");
            
            let ascii_strings = extract_ascii_strings(&buffer, min_length);
            
            // Write to output file
            let mut output_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(output_path)?;
                
            for string in &ascii_strings {
                writeln!(output_file, "{}", string)?;
            }
            
            println!("\n=== Summary ===");
            println!("Total strings found: {}", ascii_strings.len());
            println!("Results saved to: {}", output_path);
            
            // Show first 20 strings as preview
            println!("\n=== Preview (first 20 strings) ===");
            for string in ascii_strings.iter().take(20) {
                println!("{}", string);
            }
        }
    }
    
    Ok(())
}
