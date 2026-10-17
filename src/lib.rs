//! `collections` — the collection package tree, shipped as **sources**.
//!
//! `array/index.hnl` (and its siblings) use path-relative imports
//! (`from "./at" export at`) that the loader resolves **from disk**, so this
//! repo ships the tree verbatim rather than embedding text:
//! `load_hnl_file(format!("{}/src/array/index.hnl", ROOT))`.
//!
//! `ROOT` is the checkout root; the other consts are the single-file modules
//! the stdlib rows embed.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub const ARRAY__REFLECT_HNL: &str = include_str!("array/_reflect.hnl");
pub const ARRAY_AT_HNL: &str = include_str!("array/at.hnl");
pub const ARRAY_CHUNK_HNL: &str = include_str!("array/chunk.hnl");
pub const ARRAY_COMPACT_HNL: &str = include_str!("array/compact.hnl");
pub const ARRAY_CONCAT_HNL: &str = include_str!("array/concat.hnl");
pub const ARRAY_CONTAINS_HNL: &str = include_str!("array/contains.hnl");
pub const ARRAY_DEDUPE_HNL: &str = include_str!("array/dedupe.hnl");
pub const ARRAY_DEEP_FLATTEN_HNL: &str = include_str!("array/deep_flatten.hnl");
pub const ARRAY_DROP_HNL: &str = include_str!("array/drop.hnl");
pub const ARRAY_ENUMERATE_HNL: &str = include_str!("array/enumerate.hnl");
pub const ARRAY_FILL_HNL: &str = include_str!("array/fill.hnl");
pub const ARRAY_FIRST_HNL: &str = include_str!("array/first.hnl");
pub const ARRAY_FLATTEN_HNL: &str = include_str!("array/flatten.hnl");
pub const ARRAY_INDEX_HNL: &str = include_str!("array/index.hnl");
pub const ARRAY_INDEX_OF_HNL: &str = include_str!("array/index_of.hnl");
pub const ARRAY_INSERT_AT_HNL: &str = include_str!("array/insert_at.hnl");
pub const ARRAY_INTERCALATE_HNL: &str = include_str!("array/intercalate.hnl");
pub const ARRAY_INTERSPERSE_HNL: &str = include_str!("array/intersperse.hnl");
pub const ARRAY_IS_EMPTY_HNL: &str = include_str!("array/is_empty.hnl");
pub const ARRAY_LAST_HNL: &str = include_str!("array/last.hnl");
pub const ARRAY_LEN_HNL: &str = include_str!("array/len.hnl");
pub const ARRAY_PLUCK_HNL: &str = include_str!("array/pluck.hnl");
pub const ARRAY_POP_HNL: &str = include_str!("array/pop.hnl");
pub const ARRAY_PREPEND_HNL: &str = include_str!("array/prepend.hnl");
pub const ARRAY_REMOVE_AT_HNL: &str = include_str!("array/remove_at.hnl");
pub const ARRAY_REVERSE_HNL: &str = include_str!("array/reverse.hnl");
pub const ARRAY_SHIFT_HNL: &str = include_str!("array/shift.hnl");
pub const ARRAY_SLICE_HNL: &str = include_str!("array/slice.hnl");
pub const ARRAY_TAKE_HNL: &str = include_str!("array/take.hnl");
pub const ARRAY_TRANSPOSE_HNL: &str = include_str!("array/transpose.hnl");
pub const ARRAY_UNZIP_HNL: &str = include_str!("array/unzip.hnl");
pub const ARRAY_WINDOWS_HNL: &str = include_str!("array/windows.hnl");
pub const ARRAY_ZIP_HNL: &str = include_str!("array/zip.hnl");
pub const ARRAY_HNL: &str = include_str!("array.hnl");
pub const COLLECTIONS_HNL: &str = include_str!("collections.hnl");
pub const DEQUE_HNL: &str = include_str!("deque.hnl");
pub const HASH_MAP_HNL: &str = include_str!("hash_map.hnl");
pub const HASH_SET_HNL: &str = include_str!("hash_set.hnl");
pub const LIST_INDEX_HNL: &str = include_str!("list/index.hnl");
pub const LIST_IS_EMPTY_HNL: &str = include_str!("list/is_empty.hnl");
pub const LIST_LEN_HNL: &str = include_str!("list/len.hnl");
pub const LIST_PEEK_HNL: &str = include_str!("list/peek.hnl");
pub const LIST_POP_HNL: &str = include_str!("list/pop.hnl");
pub const LIST_PUSH_HNL: &str = include_str!("list/push.hnl");
pub const QUEUE_HNL: &str = include_str!("queue.hnl");
pub const STACK_HNL: &str = include_str!("stack.hnl");
pub const VEC_HNL: &str = include_str!("vec.hnl");
