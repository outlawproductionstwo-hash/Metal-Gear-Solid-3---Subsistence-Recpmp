# Metal Gear Solid 3 - Subsistence Recomp

An independent research and reverse-engineering project focused on decompiling
and recompiling **Metal Gear Solid 3: Subsistence** for educational,
preservation, and interoperability purposes.

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
