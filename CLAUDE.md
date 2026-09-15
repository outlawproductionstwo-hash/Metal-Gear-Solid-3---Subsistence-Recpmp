# MGS 3 Decompilation Project - Claude Code Agent Setup (Rust-Focused)

This directory contains tools and references for decompiling and recompiling Metal Gear Solid 3 (and potentially other PS2 games), with a primary focus on using Rust for tool development and scripting.

## Available Agent

A specialized PS2 decompilation agent has been set up for use with this project. To invoke this agent, use:

```
@ps2-agent
```

Or when spawning a new agent:
```
/agent ps2-agent
```

The PS2 agent is specialized for:
- Analyzing decompiled PS2 game code
- Understanding MIPS assembly and C decompilation outputs
- Guidance on recompilation toolchains for PS2 homebrew
- Navigating complex decompilation projects
- Setting up and using decompilation tools like Ghidra, IDA, and custom PS2 recompilation chains
- Developing Rust-based tools to aid in decompilation/recompilation processes
- Creating automation scripts for build and testing pipelines in Rust

## Project Structure

```
.
├── Astra recomp ps2/                  # Astra recompilation tools for PS2
├── Auta Decomp tool by danny 199012/  # Decompilation tools by danny
├── Claude.md based on PS2 Agent skill/ # Empty folder (placeholder)
├── Decompiled games to use as a refrence/ # Reference decompiled games
├── Decompiled games you can use as a refrence/ # Duplicate reference folder
├── Output folder of Release Once Decompiled and Recompiled into a Exe for windows through C code/ # Build outputs
├── PS 2 Recomp/                       # PS2 recompilation resources
├── PS2 Comp Agant skill/              # PS2 agent skill zips
├── Ps2 Recomp workbench/              # Workbench setup
├── The Game i want to Decompile/      # Target game files
├── ghidra ps2/                        # Ghidra configuration for PS2
└── .claude/                           # Claude Code configuration
    └── agents/
        └── ps2-agent.md               # PS2 agent configuration
```

## Using the PS2 Agent

When you need help with PS2 decompilation tasks, simply invoke the agent:

1. For general questions: Ask me to use the PS2 agent for help with your decompilation work
2. For specific tasks: Mention that you'd like the PS2 agent to assist with analyzing code, setting up toolchains, etc.
3. For complex projects: The agent can help navigate large decompiled codebases and understand complex logic

## RTK Token Optimization

This environment has RTK (Rust Token Killer) installed for token optimization. RTK automatically wraps bash commands to reduce token usage. Key commands:
- `rtk gain` - Show token savings analytics
- `rtk gain --history` - Show command usage history with savings
- `rtk discover` - Analyze Claude Code history for missed opportunities
- `rtk proxy <cmd>` - Execute raw command without filtering (for debugging)

## Rust Development Focus

As requested, our primary development language for this project will be Rust, with other languages used as needed. This means:

1. **Tool Development**: When creating new tools to aid in decompilation, analysis, or build processes, we will prioritize Rust implementations
2. **Build Scripts**: Automation scripts for building, testing, and toolchain setup will be written in Rust when possible
3. **Analysis Utilities**: Custom utilities for analyzing decompiled code or managing assets will favor Rust
4. **Interop**: When integration with existing C/MIPs decompilation tools is necessary, we'll create Rust bindings or wrappers
5. **Exceptions**: Other languages may be used when specifically required by a toolchain or when integrating with existing codebases that mandate a different language

This Rust focus provides memory safety, performance, and modern tooling benefits while still allowing flexibility for PS2-specific requirements.

## Available Skills

In addition to the PS2 agent, you have access to various skills including:
- `dataviz` - For creating charts and visualizations
- `artifact-design` - For designing artifacts
- `update-config` - For configuring Claude Code settings
- `zapier:*` - For Zapier MCP integration
- And many others available through the Skill tool

## Getting Started

To begin working with the PS2 agent on your decompilation project:
1. Invoke the agent with `@ps2-agent` or `/agent ps2-agent`
2. Describe what you need help with (analyzing code, setting up toolchain, understanding specific functions, etc.)
3. The agent will assist you with PS2-specific decompilation knowledge and tools

## Approaching Complex Decompilation Tasks

Given the size and complexity of PS2 games like MGS3, here's a recommended approach:

### Phase 1: Initial Assessment
- **Legal compliance**: Verify you have rights to work with the game
- **File examination**: Extract and examine the ISO structure
- **Tool inventory**: Identify what decompilation tools are available (Ghidra, IDA, custom PS2 tools)
- **Reference gathering**: Find similar decompiled PS2 games for reference

### Phase 2: Toolchain Setup (Rust-Focused)
- **Build environment**: Set up Rust toolchain for PS2 homebrew development
- **Analysis tools**: Create or adapt Rust-based utilities for:
  - Memory layout analysis
  - Function signature identification
  - Asset extraction and management
  - Build automation scripts
- **Integration**: Develop Rust bindings/wrappers for existing C-based decompilation tools

### Phase 3: Systematic Decompilation
- **Entry points**: Identify game entry points and initialization code
- **Module breakdown**: Separate engine, game logic, rendering, audio, input systems
- **Incremental progress**: Focus on subsystems one at a time
- **Validation**: Regularly test recompiled modules for correctness
- **Documentation**: Maintain detailed notes on findings and decisions

### Phase 4: Rust Tool Development
- **Automation scripts**: Create Rust scripts for repetitive tasks
- **Analysis dashboards**: Build visualization tools for code coverage and progress
- **Testing frameworks**: Develop property-based testing for decompiled code
- **Performance analysis**: Create profiling tools for performance bottlenecks

**Note**: Please ensure you have legal rights to work with any game files you're decompiling, and respect all applicable laws and licenses.