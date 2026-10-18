# Contributing

## Code guidelines

- Name every `PhantomData` field `_marker`, e.g. `_marker: PhantomData<T>`. Don't use `_phantom`, `__phantom` or other variants.
- When a `const fn` exists alongside a non-const version of the same function, give the `const fn` a `const_` prefix, e.g. `Key::new` / `Key::const_new`.
- Use `HashMap`, `HashSet`, `Mutex` and `RwLock` from `crate::util` instead of `std` (enforced by `clippy.toml`).
