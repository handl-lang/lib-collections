# lib-collections

`collections` — the collection package tree (`vec`, `list`, `queue`, `stack`,
`deque`, `hash_map`, `hash_set`, `array`), shipped as **sources**.

Unlike the other ports this one is not a single `index.hnl`: `array/index.hnl`
uses path-relative imports (`from "./at" export at`) that the loader resolves
from disk, so the tree must exist verbatim. Consumers read it through
`handl_lib_collections::ROOT`.

Moved from `stdlib/handl-packages/collections/` (ADR-0128).
