# `wasm-postprocess`

Various tools for postprocessing & optimizing WebAssembly modules.

## `wasm-multi-value-reverse-polyfill`

Forked from <https://github.com/vmx/wasm-multi-value-reverse-polyfill>.

### Usage

```shell
$ wasm-multi-value-reverse-polyfill module.wasm 'func1 i32 i32' 'func2 f32 f64' ...
```

## `wasm-set-start`

Sets the given function as the module's [start function] so it will run
automatically upon instantiation.

### Usage

```shell
$ wasm-set-start module.wasm func_name
```

[start function]: https://webassembly.github.io/spec/core/syntax/modules.html#syntax-start
