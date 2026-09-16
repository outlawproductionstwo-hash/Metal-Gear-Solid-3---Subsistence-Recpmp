use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug, Default)]
struct FunctionInfo {
    address: u64,
    name: String,
    size: u32,
    signature: Option<String>,
    is_library: bool,
    confidence: f32,
}

// Simple library function detection
fn is_library_function(name: &str) -> bool {
    let library_prefixes = [
        "printf", "malloc", "free", "memcpy", "memset", "strcpy", "strncpy",
        "strcmp", "strlen", "strcat", "strncat", "sprintf", "fopen", "fclose",
        "fread", "fwrite", "fseek", "ftell", "exit", "abort", "assert",
        "__printf", "__malloc", "__free", "__memcpy", "__memset"
    ];

    library_prefixes.iter().any(|&prefix| name.starts_with(prefix))
}

// Simple confidence calculation based on function characteristics
fn calculate_confidence(func: &FunctionInfo) -> f32 {
    let mut confidence = 0.5f32; // Base confidence

    // Increase confidence for non-library functions
    if !func.is_library {
        confidence += 0.2;
    }

    // Increase confidence for reasonable function sizes
    if func.size > 0 && func.size < 10000 {
        confidence += 0.2;
    }

    // Increase confidence if we have a signature
    if func.signature.is_some() {
        confidence += 0.1;
    }

    // Clamp between 0.0 and 1.0
    if confidence > 1.0 { confidence = 1.0; }
    if confidence < 0.0 { confidence = 0.0; }
    confidence
}

fn analyze_ghidra_export(data: &[u8]) -> io::Result<Vec<FunctionInfo>> {
    let text = String::from_utf8_lossy(data);
    let mut functions = Vec::new();
    let lines = text.lines();
    let mut first_line = true;

    for line in lines {
        if first_line {
            first_line = false;
            // Skip header line if it contains column names
            if line.contains("Address") && line.contains("Name") {
                continue;
            }
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 3 {
            // Parse address (remove 0x prefix if present)
            let addr_str = parts[0].trim_start_matches("0x").trim();
            let size_str = parts[2].trim();

            if let (Ok(address), Ok(size)) = (
                u64::from_str_radix(addr_str, 16),
                size_str.parse::<u32>()
            ) {
                let mut func = FunctionInfo {
                    address,
                    name: parts[1].trim().to_string(),
                    size,
                    ..Default::default()
                };

                // Optional signature (4th column)
                if parts.len() > 3 {
                    let signature = parts[3].trim();
                    if !signature.is_empty() {
                        func.signature = Some(signature.to_string());
                    }
                }

                func.is_library = is_library_function(&func.name);
                func.confidence = calculate_confidence(&func);
                functions.push(func);
            }
        }
    }

    Ok(functions)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input> <output>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];

    println!("MGS 3 Function Analyzer");
    println!("======================");
    println!("Input: {}", input);
    println!("Output: {}", output);

    // Read input file
    let mut input_file = File::open(input)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    println!("Read {} bytes from input file", buffer.len());

    // Analyze as Ghidra export
    let functions = match analyze_ghidra_export(&buffer) {
        Ok(funcs) => funcs,
        Err(e) => {
            eprintln!("Error parsing Ghidra export: {}", e);
            // Fallback to basic analysis
            Vec::new()
        }
    };

    // Write output
    let mut output_file = File::create(output)?;
    writeln!(output_file, "MGS 3 Function Analysis Results")?;
    writeln!(output_file, "======================")?;
    writeln!(output_file, "Input file: {}", input)?;
    writeln!(output_file, "File size: {} bytes", buffer.len())?;
    writeln!(output_file, "Functions found: {}", functions.len())?;

    if !functions.is_empty() {
        writeln!(output_file, "\nFunction Details:")?;
        writeln!(output_file, "{:<10} {:<20} {:<8} {:<10} {:<12} {}",
                 "Address", "Name", "Size", "Library", "Confidence", "Signature")?;
        writeln!(output_file, "{:-<80}", "")?;

        for func in functions.iter().take(20) { // Show first 20 functions
            let lib_str = if func.is_library { "Yes" } else { "No" };
            let conf_str = format!("{:.2}", func.confidence);
            let sig_str = func.signature.as_deref().unwrap_or("");
            writeln!(output_file, "{:<10} {:<20} {:<8} {:<10} {:<12} {}",
                     format!("0x{:x}", func.address),
                     func.name,
                     func.size,
                     lib_str,
                     conf_str,
                     sig_str)?;
        }

        if functions.len() > 20 {
            writeln!(output_file, "\n... and {} more functions", functions.len() - 20)?;
        }

        // Summary statistics
        let library_count = functions.iter().filter(|f| f.is_library).count();
        let high_confidence_count = functions.iter().filter(|f| f.confidence > 0.7).count();
        writeln!(output_file, "\nSummary:")?;
        writeln!(output_file, "Library functions: {}", library_count)?;
        writeln!(output_file, "High confidence functions (>0.7): {}", high_confidence_count)?;
    } else {
        // Fallback to basic analysis if no functions parsed
        let zero_count = buffer.iter().filter(|&&b| b == 0).count();
        let printable_count = buffer.iter().filter(|&&b| b >= 32 && b <= 126).count();

        writeln!(output_file, "\nBasic Analysis (No functions parsed):")?;
        writeln!(output_file, "Zero bytes: {}", zero_count)?;
        writeln!(output_file, "Printable ASCII bytes: {}", printable_count)?;
    }

    writeln!(output_file, "\nAnalysis complete.")?;

    println!("Analysis complete. Found {} functions.", functions.len());
    println!("Results written to: {}", output);
    Ok(())
}