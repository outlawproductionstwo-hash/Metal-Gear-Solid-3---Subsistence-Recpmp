# Metal Gear Solid 3 - Subsistence Recomp

[![Default branch](https://img.shields.io/badge/default%20branch-master-2ea44f)](https://github.com/outlawproductionstwo-hash/Metal-Gear-Solid-3---Subsistence-Recpmp/tree/master)
[![License](https://img.shields.io/github/license/outlawproductionstwo-hash/Metal-Gear-Solid-3---Subsistence-Recpmp)](LICENSE)
[![Last commit](https://img.shields.io/github/last-commit/outlawproductionstwo-hash/Metal-Gear-Solid-3---Subsistence-Recpmp/master)](https://github.com/outlawproductionstwo-hash/Metal-Gear-Solid-3---Subsistence-Recpmp/commits/master)
[![Repository size](https://img.shields.io/github/repo-size/outlawproductionstwo-hash/Metal-Gear-Solid-3---Subsistence-Recpmp)](https://github.com/outlawproductionstwo-hash/Metal-Gear-Solid-3---Subsistence-Recpmp)

An independent research and reverse-engineering project for **MGS3 —
Metal Gear Solid 3: Subsistence**. The project focuses on decompilation and
recompilation for educational, preservation, and interoperability purposes.

## Development assistance

This project uses **Claude** and **GitHub Copilot** as development assistants
for repository organization, documentation, research support, tooling, and
code-generation tasks. All project direction, technical decisions, review, and
published changes are managed by the project owner.

The project name follows the target game and the recompilation goal:

- **Metal Gear Solid 3 - Subsistence** is the game being studied.
- **Recomp** describes the long-term goal of producing a functionally
  equivalent, portable reimplementation from legally obtained materials.

## Project status

**Overall progress: 1% — project setup and research phase**

`[##------------------] 1%`

| Area | Progress | Status |
| --- | ---: | --- |
| Repository and build setup | 10% | Complete |
| PS2 executable analysis | 0% | Not started |
| Function identification and matching | 0% | Not started |
| Data and asset format research | 0% | Not started |
| Recompilation/runtime support | 0% | Not started |
| Automated tests and validation | 0% | Not started |
| Documentation and reproducibility | 5% | In progress |

See the full weighted calculation, completed work, and next milestones in
[`PROGRESS.md`](PROGRESS.md). Percentages are estimates for visibility and are
not a measurement of game completion or compatibility.

## Roadmap

The project will advance through these broad stages:

- [x] Create the repository structure and safe contribution rules
- [x] Add project documentation, progress tracking, and licensing
- [ ] Document the target executable format and memory layout
- [ ] Establish a repeatable PS2 analysis workflow
- [ ] Identify and verify initial functions or data formats
- [ ] Build the first project-specific recompilation support
- [ ] Add automated validation for reconstructed behavior
- [ ] Document reproducible setup and release milestones

## Current focus

The current focus is organizing the analysis workflow and documenting the
target executable before assigning progress to decompilation or recompilation
milestones. The percentage is intentionally conservative until there is
verifiable reverse-engineering evidence.

## Contributing changes

Keep changes small and descriptive:

1. Add source code, scripts, notes, or reproducible documentation.
2. Do not add original game media, ROMs, copyrighted assets, or downloaded
   archives.
3. Update [`PROGRESS.md`](PROGRESS.md) when a measurable milestone changes.
4. Commit related work together and push it to the `master` branch.

## Repository contents

| Included | Excluded |
| --- | --- |
| Original source, tools, notes, and documentation | ROMs and disc images |
| Rust utilities and reproducible scripts | Copyrighted game assets |
| Analysis results that can be legally shared | Downloaded tool archives |
| Progress and project metadata | Generated binaries and local secrets |

## Scope

- Analyze the PlayStation 2 executable and supporting data formats.
- Document discoveries and reproducible decompilation workflows.
- Develop supporting tools, preferably in Rust where practical.
- Reconstruct and test code incrementally.
- Keep the repository focused on source code, scripts, notes, and reproducible
  project metadata.

## Repository policy

Original game media, ROMs, disc images, copyrighted assets, generated binaries,
and downloaded tool archives are intentionally excluded from Git. Do not add
or upload those files. Use legally obtained files locally and keep any
machine-specific paths outside the repository.

## Working with the project

This repository is currently being organized around the PS2 recompilation
toolchain and analysis notes. New work should be committed in small,
descriptive changes so the history shows how the project progresses.

Useful areas will be added as the decompilation develops:

```text
docs/       Research notes and format documentation
tools/      Rust utilities and automation
src/        Reconstructed or matching source code
tests/      Reproducible analysis and behavior tests
```

## Legal notice

This is a non-commercial research project. Metal Gear Solid 3 and Subsistence
are trademarks and copyrighted works of their respective rights holders. This
repository does not distribute the original game or its copyrighted assets.

The original project code and documentation in this repository are licensed
under the [MIT License](LICENSE). The MIT License does not apply to the game,
ROMs, copyrighted assets, third-party tools, or other materials that are not
original work by this project.
