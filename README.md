# purescript-foldable-traversable

[![Latest release](http://img.shields.io/github/release/purescript/purescript-foldable-traversable.svg)](https://github.com/purescript/purescript-foldable-traversable/releases)
[![Build status](https://github.com/purescript/purescript-foldable-traversable/workflows/CI/badge.svg?branch=master)](https://github.com/purescript/purescript-foldable-traversable/actions?query=workflow%3ACI+branch%3Amaster)
[![Pursuit](https://pursuit.purescript.org/packages/purescript-foldable-traversable/badge)](https://pursuit.purescript.org/packages/purescript-foldable-traversable)

Classes for foldable and traversable data structures.

## Installation

```
spago install foldable-traversable
```

## Documentation

Module documentation is [published on Pursuit](http://pursuit.purescript.org/packages/purescript-foldable-traversable).

## Rust tests

Run `bin/test -c` to rebuild the sibling `purust` compiler, clear this package's
caches and generate fresh TAST and Rust. `bin/test` skips the compiler rebuild.
Both use the compiler's local Spago dependency and select the sibling TAST-enabled
PureScript fork; `PURS=/path/to/purs` overrides that selection.

`Test.Main` preserves the complete 595-line upstream/Go suite, including its
20,000-element stack-safety checks, fold and traversal defaults, indexed laws,
`Bifoldable`, `Bitraversable` and `Foldable1` tests. `Test.Runner` also runs eight
additional groups covering empty arrays, fold direction, indexed callbacks,
Cartesian choice order, effects that stop on an exception, both accumulation
and scan directions, and `Traversable1` instances and defaults. Effect traversal
is checked for deferred execution, left-to-right order and replay, twice on
100,000 elements.

Three Rust tests call all four FFI functions directly, independently of compiler
intrinsics. They check captured callbacks, argument and invocation order, input
preservation, empty arrays, generic Identity and Effect traversal, replay and
ordered Cartesian products. The balanced traversal follows the upstream
partitioning so its applicative expression has logarithmic depth.

The runner checks the process status, complete stdout and empty stderr, and
limits the PureScript suite to 60 seconds. The separate upstream benchmark
function remains available in `Test.Main`; the test runner does not invoke it.
