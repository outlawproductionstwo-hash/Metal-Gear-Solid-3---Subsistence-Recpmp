# PS2 Decompilation Project - Dependencies Analysis

This document outlines all dependencies required for the PS2 decompilation workflow based on analysis of the available tools in the project.

## Overview

The PS2 decompilation project relies on several interconnected toolchains, each with its own dependencies. Understanding these requirements is crucial for setting up a functional decompilation environment.

## Core Toolchain Dependencies

### 1. PS2Recomp/AstraRecomp Recompilation Toolchain
**Primary tools**: PS2Recomp (main), AstraRecomp (alternative/Vita-focused)

#### Build System Requirements
- **CMake**: Version 3.18+ (3.21+ recommended)
- **Compiler**: 
  - MSVC Build Tools 2022 (Windows)
  - GCC/Clang (Linux/macOS)
  - VitaSDK toolchain (for Vita targets)
- **Build Generator**: Ninja (recommended) or Make

#### Library Dependencies (automatically fetched via CMake's FetchContent)
- **ELFIO**: For ELF file parsing (https://github.com/serge1/ELFIO)
- **toml11**: For TOML configuration parsing (https://github.com/ToruNiina/toml11)
- **fmt**: For formatting library (https://github.com/fmtlib/fmt)
- **libdwarf**: For DWARF debugging information (https://github.com/davea42/libdwarf-code)
- **rabbitizer**: For MIPS/R5900 instruction decoding (https://github.com/Decompollaborate/rabbitizer)
- **nlohmann_json**: For JSON handling in analyzer (https://github.com/nlohmann/json)
- **raylib**: For runtime visualization (https://github.com/raysan5/raylib)
- **imgui + rlImGui**: For debug UI (when enabled)
- **FFmpeg**: For MPEG video decoding (optional, automatic download on Windows)

#### Platform-Specific Dependencies
- **VitaSDK**: Required for Vita cross-compilation (sets up toolchain automatically when VITASDK env var is defined)
- **Android NDK/SDK**: Required for Android builds
- **Windows SDK**: Required for Windows builds (part of MSVC Build Tools)
- **OpenSLES, SDL2, taihen_stub, Sce*_stub libraries**: Vita-specific dependencies (handled by VitaSDK when targeting Vita)

### 2. Ghidra with PS2 EmotionEngine Extensions
**Primary tool**: Ghidra + PS2 extensions for static analysis

#### Core Requirements
- **Java JDK**: Version 21+ (Temurin/OpenJDK)
  - Note: Does NOT need to be on PATH; scripts set JAVA_HOME per process
- **Ghidra**: Version 12.0.4+ (must match the PS2 extension version)
- **PS2 EmotionEngine Extension**: `ghidra-emotionengine-reloaded` (must match Ghidra version exactly)

#### Ghidra-Specific Notes
- **Language ID**: `r5900:LE:32:default` (MIPS-R5900, PS2 variant, little-endian, 32-bit)
- **Extension Location**: Must be installed in `<Ghidra_Install_Dir>/Ghidra/Extensions/`
  - Note: The top-level `Extensions/` directory is only for distributable zips
- **Headless Automation**: When using `analyzeHeadless.bat`, properties follow format: `"title + space + second argument"`

### 3. PS2 Recomp Workbench Automation
**Supporting system**: Automation scripts for build/extraction/Ghidra integration

#### Requirements
- **Windows Batch Files**: `.bat` scripts (requires Windows CMD.EXE)
- **Python 3.x**: Standard library only (no external packages needed)
  - Used for: ELF extraction, configuration normalization, TOML validation
- **Environment Variables**: Configurable via:
  1. Environment variables (`PS2X_REPO`, `PS2X_GAMES_ROOT`, etc.)
  2. `scripts/paths.properties` (user overrides, git-ignored)
  3. Auto-detection based on script location

#### Specific Script Dependencies
- `extract_elf.py`: Python standard library only
- `normalize_config.py`: Python standard library only
- `validate_toml.py`: Python standard library only
- `create_functions_at.java`: Requires JDK for compilation (but not runtime for end users)

### 4. PS2 Recomp Agent Skill
**Knowledge base**: Documentation and reference materials (no runtime dependencies)
- Contains 13+ reference guides, databases, and examples
- All content is reference/documentation only
- No installation or runtime requirements

## Dependency Summary by Category

### Absolute Requirements (Must be installed manually)
1. **CMake** (3.18+)
2. **Java JDK** (21+ for Ghidra)
3. **Python** (3.x, stdlib only)
4. **Git** (for fetching dependencies via CMake's FetchContent)
5. **Visual Studio Build Tools 2022** (Windows) OR **GCC/Clang** (Linux/macOS)
6. **Ghidra** (12.0.4+) + matching **PS2 EmotionEngine extension**

### Automatic Dependencies (Handled by build systems)
These are automatically downloaded and built by CMake's FetchContent:
- ELFIO, toml11, fmt, libdwarf, rabbitizer, nlohmann_json
- raylib, imgui, rlImGui (conditional)
- FFmpeg (Windows: prebuilt binaries; Linux/macOS: pkg-config)

### Platform-Specific Dependencies (Optional, based on build targets)
1. **VitaSDK** (for Vita cross-compilation)
2. **Android NDK/SDK** (for Android builds)
3. **Additional Vita SDK libraries**: When targeting Vita, requires specific stub libraries that come with VitaSDK

## Environment Setup Recommendations

### Windows Development Environment (Recommended)
1. Install:
   - Visual Studio Build Tools 2022 (with C++ workload)
   - CMake (3.21+)
   - Java JDK 21+ (Temurin)
   - Python 3.x
   - Git
2. Optionally install VitaSDK if targeting Vita
3. Clone/extract all tool repositories
4. Use the provided workbench scripts (`scripts/paths.bat`) for environment detection

### Cross-Platform Notes
- All build systems use CMake, making them portable across Windows/Linux/macOS
- The workbench scripts are Windows-specific (.bat files), but equivalent functionality could be created in shell scripts
- Python scripts in workbench are cross-platform (stdlib only)
- Ghidra is cross-platform Java application

## Common Issues and Solutions

### CMake Generator Issues on Windows
- **Problem**: `cmake -G "Visual Studio 17 2022"` fails to find VS instances
- **Solution**: Use Ninja generator with `vcvars64.bat` environment:
  ```cmd
  cmd /k "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
  cmake -G Ninja ..
  ```

### Batch File Error Handling
- **Problem**: `.bat` files ending in `echo` always report success
- **Solution**: Proper error handling in scripts:
  ```bat
  set RC=%ERRORLEVEL%
  echo ===== BUILD EXITCODE=%RC% =====
  exit /b %RC%
  ```

### Stack Overflow in Tests
- **Problem**: `ps2x_tests.exe` stack overflow in Debug builds
- **Solution**: Increase stack size:
  ```bat
  editbin /STACK:16777216 ps2x_tests.exe
  ```
  Or add to CMake: `target_link_options(ps2x_tests PRIVATE /STACK:16777216)`

### Ghidra Project Naming
- **Problem**: Headless scripts fail if Ghidra project name doesn't match game ID
- **Solution**: Name Ghidra project files to match `<GAME_ID>.gpr` and `<GAME_ID>.rep`

### Delayed Expansion in Loops
- **Problem**: `.bat` variables not expanding correctly in `for` loops
- **Solution**: Enable delayed expansion before the loop:
  ```bat
  setlocal enabledelayedexpansion
  for %%L in (...) do (
      set VAR=!VAR!%%L
  )
  ```

## Rust Development Environment (Project Focus)

As requested for Rust focus in this project, the following Rust-specific dependencies would be needed for developing analysis/tools:

### Rust Toolchain
- **rustc** and **cargo** (via rustup)
- **Target**: `rustup target add` for MIPS PS2 targets if cross-compiling Rust to run on PS2 (uncommon)
- **Host**: Standard x86_64-pc-windows-msvc / x86_64-unknown-linux-gnu for analysis tools

### Common Rust Dependencies for Decompilation Tools
- **serde** + **serde_json**: For JSON parsing (Ghidra exports, config files)
- **toml**: For TOML configuration files
- **goblin** or **scroll**: For binary parsing (ELF, PE formats if needed)
- **image**: For handling textures/assets if needed
- **egui** or **imgui-rs**: For creating analysis GUIs
- **ratatui** or **crossterm**: For terminal-based analysis tools
- **chrono**: For timestamp handling
- **walkdir**: For directory traversal in file analysis
- **rayon**: For parallel processing when analyzing large codebases

### Build Systems for Rust Tools
- **Cargo**: Standard Rust build system
- **Optional**: `cmake` or `make` for mixed Rust/C projects

## Verification Commands

To verify your setup has the core dependencies:

```bash
# Check CMake
cmake --version

# Check Java (for Ghidra)
java -version

# Check Python
python --version

# Check Git
git --version

# Check Compiler (MSVC example)
cl

# Check that FetchContent will work (Git should be available)
git --version
```

## Next Steps for Environment Setup

1. Install the absolute requirements listed above
2. Extract or clone all tool repositories to appropriate locations
3. Test the PS2Recomp build process using the workbench scripts
4. Install Ghidra and add the PS2 EmotionEngine extension
5. Verify cross-tool integration (Ghidra analysis → PS2Recomp input)
6. For Rust focus: Install Rust toolchain and create initial analysis utilities

This environment provides a complete pipeline from ISO extraction → static analysis (Ghidra) → recompilation (PS2Recomp/AstraRecomp) → testing/validation, with opportunities throughout to develop Rust-based analysis and automation tools.