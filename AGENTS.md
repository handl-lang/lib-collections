# lib-collections

`collections` — the collection package tree (`vec`, `list`, `queue`,
`stack`, `deque`, `hash_map`, `hash_set`, `array`), shipped as
**sources**, not a single `index.hnl`: `array/index.hnl` uses
path-relative imports (`from "./at" export at`) that the loader
resolves from disk, so the tree must exist verbatim.

## Layout

| path | role |
| --- | --- |
| `src/<module>/index.hnl` | one module per collection (`vec`, `list`, `queue`, `stack`, `deque`, `hash_map`, `hash_set`, `array`) |
| `src/array/*.hnl` | the largest module — one arrow per file (`at`, `slice`, `zip`, `flatten`, …) plus `*.test.hnl` |
| `atoms.yaml` | atom registry |
| `pkg.hnl`, `scripts/hnl-gate.sh` | manifest (8 modules) + `.hnl` gate |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign |

## Contract

- Consumers vendor `pkg.hnl` + `src/**/*.hnl` via
  `handl-pkg-vendor` into `OUT_DIR` (`plugin-lib-stdlib`,
  `handl-runtime`, `plugin-ext-refinement` do this) and load the files
  from the vendored path — the `handl-lib-collections` const crate is
  retired.
- Keep path-relative imports working: do not flatten `array/` into a
  single file.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
