# PS2 Decompilation Project - Installation Guide (Windows)

This document provides step-by-step instructions for setting up the PS2 decompilation environment on Windows.

## Prerequisites (Absolute Requirements)

The following must be installed manually:

1. **Visual Studio Build Tools 2022** (with C++ workload)
   - Download from: https://visualstudio.microsoft.com/downloads/
   - Select "Build Tools for Visual Studio 2022" → Individual components → Ensure "MSVC v143 - VS 2022 C++ x64/x86 build tools" and "Windows 10 SDK" are selected.

2. **CMake** (version 3.21+ recommended)
   - Download from: https://cmake.org/download/
   - Install and ensure it's added to PATH.

3. **Java JDK** (version 21+ for Ghidra)
   - Download Temurin JDK 21 from: https://adoptium.net/temurin21/
   - Install and note the installation path (e.g., `C:\Program Files\Eclipse Temurin\jdk-21.0.2+13`).
   - The scripts will set `JAVA_HOME` per process, but having it in PATH is helpful.

4. **Python** (3.x, standard library only)
   - Download from: https://www.python.org/downloads/windows/
   - Ensure "Add Python to PATH" is checked during installation.

5. **Git**
   - Download from: https://git-scm.com/download/win
   - Install with default options (includes Git Bash).

6. **Ghidra** (12.0.4+) with matching **PS2 EmotionEngine extension**
   - Download Ghidra 12.1.3 (or later) from: https://ghidra-sre.org/
   - Download the PS2 EmotionEngine extension (`ghidra-emotionengine-reloaded`) that matches your Ghidra version from the provided zip: `ghidra ps2\ghidra-emotionengine-reloaded-main.zip` or the pre-built version in `ghidra ps2\ghidra_12.1.3_PUBLIC_20260825_ghidra-emotionengine-reloaded.zip`.
   - Installation steps:
     a. Extract Ghidra to a folder (e.g., `C:\ghidra_12.1.3_PUBLIC`).
     b. Extract the PS2 extension zip.
     c. Copy the contents of the extension's `Ghidra/Extensions` folder to `<Ghidra_Install_Dir>\Ghidra\Extensions\`.
        Example: Copy `C:\path\to\extracted\ghidra-emotionengine-reloaded\Ghidra\Extensions\ghidra-emotionengine-reloaded` to `C:\ghidra_12.1.3_PUBLIC\Ghidra\Extensions\`.

## Optional Platform-Specific Dependencies

These are only needed if you plan to compile for specific targets:

- **VitaSDK** (for Vita cross-compilation)
  - Follow instructions at: https://vitadev.github.io/vitasdk/
  - Ensure the `VITASDK` environment variable is set to the VitaSDK installation path.

- **Android NDK/SDK** (for Android builds)
  - Install Android Studio (which includes SDK) and download the NDK via SDK Manager.
  - Set `ANDROID_NDK_HOME` and `ANDROID_SDK_ROOT` environment variables.

## Environment Setup Steps

### 1. Install Prerequisites
Install all the absolute requirements listed above.

### 2. Extract Tool Repositories
Extract the provided zip files to appropriate locations (or keep them in place):

- `PS 2 Recomp\PS2Recomp-main.zip` → Extract to `PS2Recomp-main` (or similar)
- `Astra recomp ps2\AstraRecomp-main.zip` → Extract to `AstraRecomp-main`
- `ghidra ps2\ghidra-emotionengine-reloaded-main.zip` → Extract for Ghidra extension (see above)
- `Ps2 Recomp workbench\ps2recomp-workbench-main.zip` → Extract to `ps2recomp-workbench-main`
- `PS2 Comp Agant skill\ps2-recomp-Agent-SKILL-main.zip` → Extract for reference (no build needed)

### 3. Set Up Environment (Using Workbench Scripts)
The PS2 Recomp Workbench provides scripts to help manage paths and build processes.

- Navigate to the extracted workbench folder: `ps2recomp-workbench-main`
- Review the `scripts\paths.bat` and `scripts\paths.properties.example` for environment variable configuration.
- You can set environment variables globally or create a `scripts\paths.properties` file (git-ignored) to override defaults.

### 4. Build PS2Recomp (Using Workbench Scripts)
The workbench provides batch files to automate the build process.

- Open a command prompt and navigate to the PS2Recomp extracted folder (e.g., `PS2Recomp-main`).
- Run the workbench build script (if available) or use the following general steps:

  ```cmd
  REM Ensure you are in the PS2Recomp-main directory
  cmake -S . -B build -G Ninja
  cmake --build build --config Release
  ```

  The workbench may have specific scripts; check for `build.bat` or similar.

### 5. Install and Configure Ghidra
- Run Ghidra by executing `ghidraRun.bat` in the Ghidra installation directory.
- Verify the PS2 EmotionEngine extension is installed: `Window → Extensions → Extensions Manager` should show `ghidra-emotionengine-reloaded`.
- When creating a new project, ensure you select the correct language: `r5900:LE:32:default` (MIPS-R5900, PS2 variant, little-endian, 32-bit).

### 6. Verify Setup
Run the following commands to verify core dependencies:

```cmd
cmake --version
java -version
python --version
git --version
cl  // Should show MSVC version if Build Tools installed correctly
```

### 7. (Optional) Install Rust Toolchain for Analysis Tools
As requested for Rust focus in this project:

- Install rustup from: https://rustup.rs/
- This will install `rustc` and `cargo`.
- You can then create Rust-based analysis tools using Cargo.

## Verification Commands

After installation, verify your setup:

```cmd
:: Check CMake
cmake --version

:: Check Java (for Ghidra)
java -version

:: Check Python
python --version

:: Check Git
git --version

:: Check Compiler (MSVC)
cl

:: Check that FetchContent will work (Git should be available)
git --version
```

## Next Steps

1. Extract the game ISO from `The Game i want to Decompile\Metal Gear Solid 3 - Subsistence (USA) (EnEs) (Disc 1) (Subsistence)\Metal Gear Solid 3 - Subsistence (USA) (En,Es) (Disc 1) (Subsistence).iso` using the workbench's `extract_elf.py` script or similar.
2. Load the extracted ELF files into Ghidra with PS2 extensions for initial analysis.
3. Use the Agent Skill resources for guidance on PS2 architecture and decompilation patterns.
4. Consider creating Rust-based tools to assist with analysis automation (e.g., parsing Ghidra exports, build automation).

## Troubleshooting

### Common Issues

- **CMake Generator Issues**: If using Visual Studio generator fails, try Ninja with the vcvars environment:
  ```cmd
  cmd /k "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
  cmake -G Ninja ..
  ```

- **Batch File Error Handling**: If scripts fail silently, ensure they properly propagate errorlevels.

- **Ghidra Project Naming**: For headless scripts, name Ghidra project files to match `<GAME_ID>.gpr` and `<GAME_ID>.rep`.

- **Stack Overflow in Tests**: Increase stack size for Debug builds if needed:
  ```cmd
  editbin /STACK:16777216 ps2x_tests.exe
  ```

## Notes

- The workbench scripts are Windows-specific (.bat files). For cross-platform use, equivalent shell scripts could be created.
- Python scripts in the workbench use only the standard library, so they are cross-platform.
- Ghidra is a cross-platform Java application.

This environment provides a complete pipeline from ISO extraction → static analysis (Ghidra) → recompilation (PS2Recomp/AstraRecomp) → testing/validation.

Co-Authored-By: Claude Code <noreply@anthropic.com>