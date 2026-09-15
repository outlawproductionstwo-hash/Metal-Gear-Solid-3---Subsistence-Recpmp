# PS2 Decompilation Project - Tools Mapping and Analysis

This document provides a comprehensive analysis of all tools available in the PS2 decompilation project, explaining what each tool does and how they fit together in a typical PS2 game decompilation workflow.

## Overview

The project contains several major tool categories:
1. **Recompilation Toolkits** - PS2Recomp, AstraRecomp
2. **Reverse Engineering Tools** - Ghidra with PS2 extensions
3. **Agent Frameworks** - PS2 Recomp Agent Skill
4. **Workbench/Support Tools** - PS2 Recomp Workbench
5. **Reference Materials** - Documentation and reference files
6. **Game Files** - Target ISO for decompilation

## Detailed Tool Analysis

### 1. PS2Recomp (PS2 Recompiler)
**Location**: `PS 2 Recomp/PS2Recomp-main.zip`
**Purpose**: Core recompilation toolchain for converting PS2 ELF executables to native executables (Windows/Linux/Vita/etc.)

**Key Components**:
- **ps2xAnalyzer**: ELF analysis tool that extracts information from PS2 executables
- **ps2xRecomp**: Main recompiler that converts MIPS/R5900 code to native code
- **ps2xIOP**: Input/Output Processor subsystem handling
- **ps2xRuntime**: Runtime environment providing PS2 system emulation
- **ps2xStudio**: Debugger/visualization interface
- **ps2xTest**: Test suite for validating recompilation accuracy

**Workflow Role**: Primary tool for converting analyzed PS2 code into runnable native executables. Used after initial analysis with Ghidra or similar tools.

**File Types Handled**: PS2 ELF executables, generates native executables + runtime libraries

### 2. AstraRecomp
**Location**: `Astra recomp ps2/AstraRecomp-main.zip`
**Purpose**: Alternative PS2 recompilation toolkit with focus on Vita porting and performance optimization

**Key Components**:
- **Core CPU/Memory/GS/IOP emulation** similar to PS2Recomp
- **Vita-specific target support** for porting PS2 games to PSVita
- **AOT (Ahead-of-Time) compilation** capabilities
- **Performance analysis and optimization tools**
- **Integration documentation** with PS2Recomp

**Workflow Role**: Alternative or complementary recompilation toolkit, particularly useful for Vita targets and performance-focused recompilation.

### 3. Ghidra with PS2 Extensions
**Location**: `ghidra ps2/ghidra-emotionengine-reloaded-main.zip`
**Purpose**: NSA's Ghidra reverse engineering framework enhanced with PS2-specific support

**Key Components**:
- **Processor Modules**: R5900 (PS2's custom MIPS variant), COP0, COP1, COP2, VU (Vector Units)
- **Language Specs**: .sinc files for instruction set definitions
- **Parsers**: .cspec/.pspec/.slaspec for program structure
- **Ghidra Scripts**: Python/Java scripts for automating PS2-specific analysis tasks
  - Function export/import utilities
  - Memory map analyzers
  - Overlay system handlers
  - Save state importers

**Workflow Role**: Primary tool for static analysis of PS2 ELF executables. Used to:
- Disassemble and decompile MIPS/R5900 code
- Identify functions, data structures, and memory maps
- Add comments and labels to facilitate understanding
- Export analysis data for use with recompilation tools

### 4. PS2 Recomp Agent Skill
**Location**: `PS2 Comp Agant skill/ps2-recomp-Agent-SKILL-main.zip`
**Purpose**: Comprehensive knowledge base and framework for PS2 game decompilation projects

**Key Components**:
- **Documentation Resources** (13+ detailed guides):
  - PS2 Hardware Bible
  - MIPS R5900 ISA reference
  - PS2Recomp pipeline explanation
  - Runtime syscalls and stubs guide
  - Ghidra/GhidraMCP integration guide
  - Game porting playbook
  - PS2 code patterns reference
  - Infinite knowledge base concepts
- **Reference Databases**: Extensive collections of:
  - Memory maps
  - Register definitions
  - SDK function signatures
  - Syscall tables
  - Vector Unit instructions
  - PS2 architecture overviews
- **Example Templates**:
  - Game override templates (C++)
  - TOML configuration templates
- **Integration Scripts**:
  - GhidraMCP installer
  - VIF/GIF surgeon (graphics pipeline tools)
  - Project state management
  - Game agent templates

**Workflow Role**: Knowledge base and procedural guide for conducting PS2 decompilation projects. Provides:
- Educational resources for understanding PS2 architecture
- Reference materials for identifying functions and structures
- Integration guidelines for combining tools effectively
- Best practices for game porting and recompilation

### 5. PS2 Recomp Workbench
**Location**: `Ps2 Recomp workbench/ps2recomp-workbench-main.zip`
**Purpose**: Practical workflow automation and management system for PS2Recomp-based projects

**Key Components**:
- **Documentation Guides**:
  - Environment setup instructions
  - Game assets layout standards
  - Known issues and pending fixes tracking
  - Recompilation workflow documentation
- **Automation Scripts** (.bat and .py):
  - ELF extraction from ISOs/PS2 images
  - Build automation for tools and games
  - Ghidra export/import automation
  - Configuration normalization
  - Path management utilities
  - Validation and testing scripts
- **Templates**:
  - Game properties configuration
  - Notes documentation templates

**Workflow Role**: Operational layer that simplifies using PS2Recomp by providing:
- Standardized project structure
- Automated build and extraction processes
- Ghidra integration workflows
- Configuration management
- Testing and validation frameworks

### 6. Supporting Tool Collections

#### Auta Decomp Tool by Danny 199012 & Ps2 Recomp Tool by Danny 199012
**Status**: Currently empty folders, likely placeholders for additional decompilation tools that may be added later.

#### Reference Folders
**Status**: Currently empty, intended for storing:
- Previously decompiled PS2 games for reference
- Extracts from similar titles to aid in pattern recognition
- Community-sourced decompilation data

#### Output Folder
**Status**: Currently empty, intended for:
- Storing compiled executables from recompilation efforts
- Build artifacts and logs
- Distributable outputs of the decompilation/recompilation process

## Integrated Workflow Mapping

Here's how these tools typically work together in a PS2 game decompilation project:

### Phase 1: Acquisition and Preparation
1. **Source**: Obtain game ISO (in `The Game i want to Decompile/`)
2. **Extraction**: Use PS2 Recomp Workbench scripts to extract ELF files from ISO
3. **Initial Analysis**: Load extracted ELFs into Ghidra with PS2 extensions

### Phase 2: Reverse Engineering (Ghidra-Centric)
1. **Disassembly**: Use Ghidra's MIPS/R5900 processor modules to disassemble code
2. **Function Identification**: Apply patterns from Agent Skill resources to identify functions
3. **Data Structure Reconstruction**: Use memory maps and SDK references from Agent Skill
4. **Labeling and Commenting**: Annotate code with meaningful names and comments
5. **Analysis Export**: Use GhidraScripts to export function lists, memory maps, etc.

### Phase 3: Recompilation Preparation
1. **Analysis Import**: Feed Ghidra export data into PS2Recomp's analyzer
2. **Configuration**: Create TOML configs (using Agent Skill templates) for recompilation targets
3. **Stub Creation**: Generate necessary system stubs (from Agent Skill resources)
4. **Build Setup**: Use PS2Recomp Workbench or manual CMake setup

### Phase 4: Recompilation and Testing
1. **Compilation**: Use PS2Recomp or AstraRecomp to compile native executables
2. **Runtime Linking**: Link with appropriate PS2Runtime components
3. **Testing**: Run executables and validate behavior using test suites
4. **Iteration**: Refine analysis based on runtime behavior, repeat phases 2-4

### Phase 5: Optimization and Porting (Optional)
1. **Performance Analysis**: Use profiling tools to identify bottlenecks
2. **Optimization**: Apply optimizations from AstraRecomp or manual tuning
3. **Target Porting**: Adapt for different platforms (Vita, Android, etc.) using platform-specific backends

## Tool Relationships and Dependencies

```
[Game ISO] 
      ↓ (Extraction)
[ELF Files] 
      ↓ (Static Analysis)
[Ghidra Project] ←→ [Agent Skill Knowledge Base]
      ↓ (Analysis Export)
[PS2Recomp Analyzer] 
      ↓ (Recompilation Config)
[PS2Recomp/AstraRecomp Toolchain] 
      ↓ (Compilation + Runtime)
[Native Executable] 
      ↓ (Testing/Validation)
[Refined Understanding] 
      ↺ (Feedback to Analysis Phase)
```

## Recommendations for MGS3: Subsistence Decompilation

Given the target is Metal Gear Solid 3: Subsistence, here's how to approach it:

### Initial Steps
1. **Extract the ISO** using PS2 Recomp Workbench's `extract_elf.py` or similar tools
2. **Identify key ELFs**: Look for main game executable, modules, overlays
3. **Load into Ghidra**: Apply PS2 EmotionEngine extensions for proper disassembly
4. **Consult Agent Skill**: Use PS2 hardware docs, MIPS references, and game-specific patterns

### Priority Targets for Initial Analysis
Based on typical PS2 game structure:
- **Startup/Initialization code** (entry points, system setup)
- **File system/io operations** (loading assets from DVD)
- **Memory management** (allocations, DMA transfers)
- **Core game loop** (main update/render functions)
- **Graphics pipeline** (GS/VU1 interactions if visible)
- **Audio systems** (if using SPU2 or similar)

### Rust Integration Opportunities
As requested for Rust focus, consider developing:
- **Analysis utilities** in Rust for parsing Ghidra exports
- **Build automation** scripts in Rust replacing .bat files
- **Memory analysis tools** for visualizing PS2 memory usage
- **Testing frameworks** for validating decompiled code correctness
- **Performance profiling tools** for identifying bottlenecks in recompiled code

## Next Actions

To begin analyzing your MGS3: Subsistence project:

1. Extract the ISO file from `The Game i want to Decompile/`
2. Use Ghidra with PS2 extensions to start analyzing the main executable
3. Consult the Agent Skill resources for PS2-specific guidance
4. Consider creating Rust-based tools to assist with analysis automation

This tools mapping provides the foundation for understanding how to approach your complex PS2 decompilation project using the available resources.