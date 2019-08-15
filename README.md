<div align="center">
  <h1>
    <code>intersection-wasm</code>
  </h1>
  <strong>Mesh-Mesh and Triangle-Triangle Intersection tests based on the algorithm by Tomas Akenine-Möller.</strong><sup><a href="#article">[1]</a></sup>
  
  <sub>Built with 🦀 <a href="https://www.rust-lang.org" target="_blank">Rust</a> and 🕸 <a href="https://webassembly.org" target="_blank">WebAssembly</a>.</sub>

  [![GitHub version](https://badge.fury.io/gh/catenda%2Fintersection-wasm.svg)](https://badge.fury.io/gh/catenda%2Fintersection-wasm) [![npm version](https://badge.fury.io/js/intersection-wasm.svg)](https://badge.fury.io/js/intersection-wasm)
</div>

## About

Mesh-Mesh and Triangle-Triangle Intersection tests.

```typescript
/**
* Triangle/triangle intersection test
* @returns true if the triangles intersect, otherwise false
*/
const noDivTriTriIsect = (
  v0: [number, number, number],
  v1: [number, number, number],
  v2: [number, number, number],

  u0: [number, number, number],
  u1: [number, number, number],
  u2: [number, number, number],

  // not used by default
  epsilon?: number
): boolean => {...}

/**
* Mesh/mesh intersection test
* @returns true if the meshes intersect, otherwise false
*/
const meshMeshIsect => (
  // m1.length should be divisible by 9
  m1: ArrayLike<number>,
  // m2.length should be divisible by 9
  m2: ArrayLike<number>,
  // defaults to 0.000001
  epsilon?: number
): boolean => {...}
```

## 🚴 Usage

### Installation

You will need a package manager, either npm ([comes with node](https://nodejs.org/en/download)) or [yarn](https://yarnpkg.com/lang/en/docs/install).

You will also need a bundler, [webpack](https://webpack.js.org) or [Rollup](https://rollupjs.org/guide/en), configured for your project.

Then, in a terminal:

```shell
npm install intersection-wasm
# Or, yarn add intersection-wasm
```

Afterwards, import and use as follows:

```js
import * as intersection from 'intersection-wasm';

intersection.noDivTriTriIsect(
  [0.848311, 0.71034, 0.799546],
  [0.921121, 0.519029, 0.950985],
  [0, 1.751, 0],

  [-0.5, 0.8755, 0.5],
  [0.5, 0.8755, 1.5],
  [0.5, 0.8755, 0.5]
); // ← false

intersection.meshMeshIsect(
  new Float32Array([
    -140.98574829101562,
    -173.12110900878906,
    -0.9740447998046875,
    -140.98574829101562,
    -174.72113037109375,
    -0.9740447998046875,
    -140.68576049804688,
    -174.72113037109375,
    -0.9740447998046875
  ]),
  new Float32Array([
    -140.98574829101562,
    -174.72113037109375,
    -0.9740447998046875,
    -140.98574829101562,
    -174.72113037109375,
    -0.9740447998046875,
    -140.98574829101562,
    -174.72113037109375,
    -1.137430191040039,
    -140.68576049804688,
    -174.72113037109375,
    -1.137430191040039,
    -140.98574829101562,
    -174.72113037109375,
    -0.9740447998046875,
    -140.68576049804688,
    -174.72113037109375,
    -1.137430191040039
  ]),
  0.0001
); // ← true
```

## ⚙ Development

### 🛠️ Build WASM

```
wasm-pack build
```

### 🛠️ Build natively

```
cargo build
```

### 🔬 Run tests in the browser

```
cd demo && npm i && npm start
```

### 🔬 Test natively

```
cargo test
```

### 🎁 Publish to NPM

```
wasm-pack publish
```

<div id="article">1. Möller, T. (1997). A fast triangle-triangle intersection test. <i>Journal of graphics tools</i>, <i>2</i>(2), 25-30.</div>
