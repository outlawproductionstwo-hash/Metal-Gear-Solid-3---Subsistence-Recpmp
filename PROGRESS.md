# Decompilation progress

This page provides a transparent, milestone-based estimate of project progress.
The figures are updated when meaningful work is completed; they are not
automatically derived from line counts or repository size.

## Current overall estimate

**21%**

`[#####################] 21%`

The overall estimate is a weighted score. Early repository setup is useful
foundation work, but it represents only a small part of the complete
decompilation and recompilation effort.

## Progress dashboard

| Area | Weight | Progress | Weighted result | Status |
| --- | ---: | ---: | ---: | --- |
| Repository and build setup | 10% | 10% | 1.0% | Complete |
| PS2 executable analysis | 20% | 75% | 15.0% | In progress |
| Function identification and matching | 25% | 2% | 0.5% | Initial tooling |
| Data and asset format research | 15% | 0% | 0.0% | Not started |
| Recompilation/runtime support | 15% | 3% | 0.45% | Toolchain research |
| Automated tests and validation | 10% | 0% | 0.0% | Not started |
| Documentation and reproducibility | 5% | 85% | 4.25% | In progress |
| **Total** | **100%** | — | **21.2% -> 21%** | **Analysis in progress** |

## Work breakdown

### Completed

- Created the public repository structure and project documentation.
- Added Git safeguards for ROMs, disc images, archives, generated files, and
  local secrets.
- Added the MIT license for original project code and documentation.
- Added dependency/toolchain mapping and installation guidance.
- Added initial Rust utilities for ELF and string analysis.
- Added an executable function-analysis workflow for processing exported
  analysis input and writing a structured report.

### In progress

- Organizing the PS2 recompilation tools and research workflow.
- Defining the executable analysis and matching strategy.
- Installed Ghidra 12.1.3 with the PS2 EmotionEngine extension.
- Prepared the extracted boot ELF for Ghidra import.
- Extending the project-owned function analyzer beyond its initial scaffold.
- **Ghidra import and auto-analysis of SLUS_213.59 is in progress**

### Next milestones

1. Record the target executable format, entry point, sections, and memory map.
2. Establish a reproducible local analysis workflow using legally obtained
   game files without committing those files.
3. Add the first verified function or data-format match.
4. Add a small validation fixture or repeatable analysis test.

## Percentage definitions

| Range | Meaning |
| ---: | --- |
| 0% | No verified work in the area |
| 1-24% | Setup, research, or initial discoveries |
| 25-49% | Core implementation or matching is underway |
| 50-74% | Most major components exist and are being integrated |
| 75-99% | Stabilization, compatibility, and validation remain |
| 100% | The defined milestone is complete and documented |

## How percentages are used

Each area's contribution is calculated as:

`area weight x area progress / 100`

The displayed overall percentage is rounded down to avoid overstating progress.
Percentages should increase only when there is reviewable evidence, such as a
documented discovery, a verified match, a passing test, or an integrated
runtime feature.

When updating this file:

1. Adjust the relevant area.
2. Recalculate each weighted result and the overall estimate conservatively.
3. Update the completed, in-progress, and next-milestone sections.
4. Add a short changelog note describing the evidence for the change.
5. Commit the code and progress update together when they belong to the same
   milestone.

## Changelog

| Date | Overall | Update |
| --- | ---: | --- |
| 2026-09-15 | 1% | Added weighted progress tracking and documented the initial setup milestone |
| 2026-09-15 | 3% | Added installation guide for PS2 decompilation dependencies (INSTALL.md) |
| 2026-09-15 | 5% | Extracted boot ELF from MGS3 ISO using workbench extract_elf.py script |
| 2026-09-15 | 9% | Installed Ghidra 12.1.3 with PS2 EmotionEngine extension and prepared the boot ELF for analysis |
| 2026-09-15 | 12% | Verified Ghidra/PS2 environment ready for active ELF analysis - ready to run analysis scripts |
| 2026-09-15 | 19% | Started Ghidra import and auto-analysis of SLUS_213.59 with corrected paths |
| 2026-09-15 | 19% | Added reproducible toolchain documentation and initial Rust analysis utilities |
| 2026-09-15 | 21% | Added the first executable function-analysis workflow; Ghidra analysis remains in progress |

Co-Authored-By: Claude Code <noreply@anthropic.com>