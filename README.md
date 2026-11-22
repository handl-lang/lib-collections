# lib-collections

`collections` — the collection package tree (`vec`, `list`, `queue`, `stack`,
`deque`, `hash_map`, `hash_set`, `array`), shipped as **sources**.

Unlike the other ports this one is not a single `index.hnl`: `array/index.hnl`
uses path-relative imports (`from "./at" export at`) that the loader resolves
from disk, so the tree must exist verbatim. Consumers vendor `pkg.hnl` +
`src/**/*.hnl` via `handl-pkg-vendor` into `OUT_DIR` (`plugin-lib-stdlib`,
`handl-runtime`, `plugin-ext-refinement` do this) and load the files from the
vendored path — the `handl-lib-collections` const crate is retired.

## Layout

```text
src/                     the .hnl source tree
pkg.hnl                  package manifest
scripts/hnl-gate.sh      the gate
ci.sh                    bupha entry
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
