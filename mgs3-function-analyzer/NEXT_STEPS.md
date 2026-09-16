# Next Steps for Analyzing Actual SLUS_213.59 Ghidra Export

## ✅ What's Been Completed
The mgs3-function-analyzer has been successfully enhanced with:

1. **Proper Ghidra CSV Export Parser** (`src/main.rs`)
   - Structured `FunctionInfo` struct with: address, name, size, signature, is_library, confidence
   - Accurate CSV parsing (skips headers, handles hex addresses, extracts signatures)
   - Library function detection for common C functions (printf, malloc, free, etc.)
   - Confidence scoring based on function characteristics
   - Error handling and fallback to basic analysis

2. **Testing & Validation**
   - Works with simple test CSV data
   - Works with realistic PS2 function export data (including PS2-specific functions)
   - Correctly identifies library vs. game functions
   - Provides detailed output with summary statistics

## 📋 How to Get the Actual Ghidra Export

### Option 1: Use Full Ghidra Installation (Recommended)
Based on the INSTALL.md instructions:

1. **Ensure Full Ghidra is Installed**:
   - Download Ghidra 12.1.3+ from https://ghidra-sre.org/
   - Extract to a folder (e.g., `C:\ghidra_12.1.3_PUBLIC`)
   - Install PS2 EmotionEngine extension:
     - Extract `ghidra ps2\ghidra-emotionengine-reloaded-main.zip`
     - Copy extension contents to `<Ghidra_Dir>\Ghidra\Extensions\`

2. **Load & Analyze SLUS_213.59 in Ghidra**:
   - Launch Ghidra: `ghidraRun.bat` in Ghidra installation directory
   - Create/new project for SLUS_213.59
   - Import `SLUS_213.59` ELF file (from `ps2-recomp-games\SLUS_213.59\SLUS_213.59`)
   - Wait for or run auto-analysis
   - Verify PS2 EmotionEngine extension is active

3. **Export Function Table**:
   - Window → Defined Functions
   - Right-click → Export Program Functions...
   - Save as CSV (e.g., `SLUS_213.59_functions.csv`)

### Option 2: Check for Existing Export
Look for any existing function exports:
```cmd
dir /s /b "*.csv" "ps2-recomp-games\SLUS_213.59"
dir /s /b "*function*" "ps2-recomp-games\SLUS_213.59"
```

### Option 3: Use Headless Analysis (Advanced)
If you prefer command-line:
```cmd
# Path to your Ghidra installation
set GHIDRA_INSTALL=C:\path\to\ghidra_12.1.3_PUBLIC

# Run headless analysis to export functions
"%GHIDRA_INSTALL%\support\analyzeHeadless" ^
  ps2-recomp-games\SLUS_213.59\ghidra_project ^
  SLUS_213.59 ^
  -import ps2-recomp-games\SLUS_213.59\SLUS_213.59 ^
  -postScript ExportFunctions.java ^
  -deleteProject
```
*(You would need to create ExportFunctions.java script)*

## ▶️ How to Run the Function Analyzer

Once you have the CSV export:

```cmd
mgs3-function-analyzer\target\release\mgs3-function-analyzer.exe ^
  "path\to\SLUS_213.59_functions.csv" ^
  "slus_213_59_analysis.txt"
```

## 📊 Expected Output

The analyzer will produce output like:
```
MGS 3 Function Analysis Results
======================
Input file: SLUS_213.59_functions.csv
File size: 1,204,578 bytes
Functions found: 1,247

Function Details:
Address    Name                 Size     Library    Confidence   Signature
--------------------------------------------------------------------------------
0x00001000 __entry              0        No         0.80         void __entry(void)
0x00001010 main                 256      No         1.00         int main(int argc, char** argv)
0x00001110 game_init             512      No         1.00         void game_init(void)
...
0x00123456 printf                 42       Yes        0.80         int printf(const char* format, ...)
0x00123480 malloc                 36       Yes        0.80         void* malloc(size_t size)

Summary:
Library functions: 42
High confidence functions (>0.7): 981
Functions with signatures: 876
```

## 🎯 What This Provides

This analysis gives you:
- **Complete function overview** of SLUS_213.59
- **Library vs. game function separation** (helps focus on game-specific code)
- **Confidence scores** to prioritize reliable functions for analysis
- **Function signatures** where available (return types, parameters)
- **Size estimates** to identify substantial functions vs. trivial wrappers
- **Structured data** suitable for further processing or decompilation workflows

## 🔧 Troubleshooting

If you encounter issues:
1. **CSV Format Problems**: Ensure export is standard CSV with Address,Name,Size,Signature columns
2. **Missing Functions**: Check that Ghidra analysis completed successfully
3. **Parser Issues**: Verify CSV doesn't have unusual quoting or encoding
4. **Memory Issues**: Very large exports may need increased heap (adjust Java settings if using headless)

## 📁 Files Created

- `mgs3-function-analyzer\slus_213_59_functions.csv` - Sample/test export
- `mgs3-function-analyzer\slus_213_59_analysis.txt` - Sample analysis output
- `mgs3-function-analyzer\PROGRESS_SUMMARY.md` - Detailed implementation summary
- `mgs3-function-analyzer\NEXT_STEPS.md` - This file

The function analyzer is now ready to process the actual SLUS_213.59 Ghidra export once you obtain it through one of the methods above.