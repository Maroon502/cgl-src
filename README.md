# Cgl-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

## description

cgl-src crate is a *-src crate. This links Cgl libraries to executable build by cargo, but does not provide Rust bindings. [Cgl] build with [CoinUtils] ([CoinUtils-src]), [Osi] ([Clp-src]) and [Clp] ([Clp-src])(Optional) support.

By this package, you don't need to worry about installing Cgl in the system, and it can be built for **all platforms**.

The COIN-OR Cut Generation Library (Cgl) is a collection of cut generators that can be used with other COIN-OR packages that make use of cuts, such as, the mixed integer linear programming solvers [Cbc] ([Cbc-src]). Cgl uses the [Osi] ([Osi-src]) to use or communicate with a solver, such as the linear programming solvers [Clp] [Clp-src]. It does not directly call a solver.

## Usage
Just add the following to your `Cargo.toml`:

```toml
[dependencies]
cgl-src = "0.2"
```

This package does not provide bindings. Please use [coincbc-sys], [coinclp-sys] to use Cbc, Clp, e.g.

```toml
[dependencies]
coincbc-sys = { version = "0.2" }
```
## Configuration

The following Cargo features are supported:

* `default` to `with_clp` and `default_cuts` feature;
* `with_clp` to build with Clp support;
* `default_cuts` to provide all the following cut generators;

* Combinatorial cuts

    * `CglAllDifferent` to provide [CglAllDifferent] cut generator;
    <!-- * `CglBKClique` to provide [CglBKClique] cut generator; -->
    * `CglClique` to provide [CglClique] cut generator;
    * `CglKnapsackCover` to provide [CglKnapsackCover] cut generator;
    * `CglOddHole` to provide [CglOddHole] cut generator;
    <!-- * `CglOddWheel` to provide [CglOddWheel] cut generator;  -->
    * `CglZeroHalf` to provide [CglZeroHalf] cut generator;

* flow cover cuts:
    * `CglFlowCover` to provide [CglFlowCover] cut generator;

* Gomory cuts and variants:
    * `CglGomory` = to provide [CglGomory] cut generator, which requires `with_clp` feature to use [clp-src];
    * `CglGMI` to provide [CglGMI] cut generator;
    * `CglRedSplit` to provide [CglRedSplit] cut generator;
    * `CglRedSplit2` to provide [CglRedSplit2] cut generator;

* Lift and project cuts:
    * `CglLiftAndProject` to provide [CglLiftAndProject] cut generator;
    * `CglLandP` = to provide [CglLandP] cut generator, which requires `CglGomory` feature;

* Mixed integer rounding cuts and variants:
    * `CglMixedIntegerRounding` to provide [CglMixedIntegerRounding] cut generator;
    * `CglMixedIntegerRounding2` to provide [CglMixedIntegerRounding2] cut generator;
    * `CglTwomir` to provide [CglTwomir] cut generator, which requires `with_clp` feature to use [clp-src];
    * `CglResidualCapacity` to provide [CglResidualCapacity] cut generator;

* Strengthening:
    <!-- * `CglCliqueStrengthening` to provide [CglCliqueStrengthening] cut generator; -->
    * `CglDuplicateRow` to provide [CglDuplicateRow] cut generator;
    * `CglPreProcess` to provide [CglPreProcess] cut generator;
    * `CglProbing` to provide [CglProbing] cut generator;
    * `CglSimpleRounding` to provide [CglSimpleRounding] cut generator;


The package build from the source and link statically by default. It also provide the following environment variables to allow users to link to system library customly:

* `CARGO_COINUTILS_STATIC` to link to CoinUtils statically;
* `CARGO_COINUTILS_SYSTEM` to link to CoinUtils system library;
* `CARGO_OSI_STATIC` to link to Osi statically;
* `CARGO_OSI_SYSTEM` to link to Osi system library;
* `CARGO_CLP_STATIC` to link to Clp statically if `with_clp` feature is enabled;
* `CARGO_CLP_SYSTEM` to link to Clp system library if `with_clp` feature is enabled;
* `CARGO_CGL_STATIC` to link to Cgl statically;
* `CARGO_CGL_SYSTEM` to link to Cgl system library;

Set the environment variable to `1` to enable the feature. For example, to link to system library dynamically, set `CARGO_${LIB_NAME}_SYSTEM` to `1`; to link to system library statically, set both `CARGO_${LIB_NAME}_SYSTEM` and `CARGO_${LIB_NAME}_STATIC` to `1`.

## Windows and vcpkg

On Windows, if `${LIB_NAME}_SYSTEM` is set to `1`, `cgl-src` will use 
[vcpkg] to find Cgl. Before building, you must have the correct Cgl 
installed for your target triplet and kind of linking. For instance,
to link dynamically for the `x86_64-pc-windows-msvc` toolchain, install
 `cgl` for the `x64-windows` triplet:

```sh
vcpkg install cgl --triplet x64-windows
```

To link Cgl statically, install `cgl` for the `x64-windows-static-md` triplet:

```sh
vcpkg install cgl --triplet x64-windows-static-md
```

To link Cgl and C Runtime (CRT) statically, install `cgl` for the `x64-windows-static` triplet:

```sh
vcpkg install cgl --triplet x64-windows-static
```

and build with `+crt-static` option

```
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-pc-windows-msvc
```

Please see the ["Static and dynamic C runtimes" in The Rust reference](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes) for detail.

## Cross Compilation

you can compile it for the other target by providing the `--target` option to 
`cargo build`. 


| Target                               |  supported  |
|--------------------------------------|:-----------:|
| `arm-unknown-linux-gnueabi`          | ✓   |
| `arm-unknown-linux-gnueabihf`        | ✓   |
| `armv7-linux-androideabi`            | ✓   |
| `armv7-unknown-linux-gnueabi`        | ✓   |
| `armv7-unknown-linux-gnueabihf`      | ✓   |
| `armv7-unknown-linux-musleabi`       | ✓   |
| `armv7-unknown-linux-musleabihf`     | ✓   |
| `riscv64gc-unknown-linux-gnu`        | ✓   |
| `x86_64-pc-windows-gnu`              | ✓   |
| `x86_64-unknown-linux-gnu`           | ✓   |

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[CoinUtils]: https://github.com/coin-or/CoinUtils
[Osi]: https://github.com/coin-or/Osi
[Cgl]: https://github.com/coin-or/Cgl
[Clp]: https://github.com/coin-or/Clp
[Cbc]: https://github.com/coin-or/Cbc
[BCP]: https://github.com/coin-or/BCP

[CoinUtils-src]: https://github.com/Maroon502/coinutils-src
[Osi-src]: https://github.com/Maroon502/osi-src
[Cgl-src]: https://github.com/Maroon502/cgl-src
[Clp-src]: https://github.com/Maroon502/clp-src
[Cbc-src]: https://github.com/Maroon502/cbc-src
[coincbc-sys]: https://github.com/Maroon502/coincbc-sys
[coinclp-sys]: https://github.com/Maroon502/coinclp-sys

[vcpkg]: https://github.com/Microsoft/vcpkg

[documentation-img]: https://docs.rs/cgl-src/badge.svg
[documentation-url]: https://docs.rs/cgl-src
[package-img]: https://img.shields.io/crates/v/cgl-src.svg
[package-url]: https://crates.io/crates/cgl-src
[license-img]: https://img.shields.io/crates/l/cgl-src.svg
[license-url]: https://github.com/Maroon502/cgl-src/blob/master/LICENSE.md

[CglAllDifferent]: https://github.com/coin-or/Cgl/wiki/CglAllDifferent
<!-- [CglBKClique]: https://github.com/coin-or/Cgl/wiki/CglBKClique -->
[CglClique]: https://github.com/coin-or/Cgl/wiki/CglClique
[CglKnapSackCover]: https://github.com/coin-or/Cgl/wiki/CglKnapSackCover
[CglOddHole]: https://github.com/coin-or/Cgl/wiki/CglOddHole
<!-- [CglOddWheel]: https://github.com/coin-or/Cgl/wiki/CglOddWheel -->
<!-- [CglZeroHalf]: https://github.com/coin-or/Cgl/wiki/CglZeroHalf -->
[CglFlowCover]: https://github.com/coin-or/Cgl/wiki/CglFlowCover
[CglGomory]: https://github.com/coin-or/Cgl/wiki/CglGomory
<!-- [CglGMI]: https://github.com/coin-or/Cgl/wiki/CglGMI -->
[CglRedSplit]: https://github.com/coin-or/Cgl/wiki/CglRedSplit
<!-- [CglRedSplit2]: https://github.com/coin-or/Cgl/wiki/CglRedSplit2 -->
[CglLiftAndProject]: https://github.com/coin-or/Cgl/wiki/CglLiftAndProject
[CglLandP]: https://github.com/coin-or/Cgl/wiki/CglLandP
[CglMixedIntegerRounding]: https://github.com/coin-or/Cgl/wiki/CglMixedIntegerRounding
[CglMixedIntegerRounding2]: https://github.com/coin-or/Cgl/wiki/CglMixedIntegerRounding2
[CglTwomir]: https://github.com/coin-or/Cgl/wiki/CglTwomir
[CglResidualCapacity]: https://github.com/coin-or/Cgl/wiki/CglResidualCapacity
<!-- [CglCliqueStrengthening]: https://github.com/coin-or/Cgl/wiki/CglCliqueStrengthening -->
[CglDuplicateRow]: https://github.com/coin-or/Cgl/wiki/CglDuplicateRow
[CglPreProcess]: https://github.com/coin-or/Cgl/wiki/CglPreProcess
[CglProbing]: https://github.com/coin-or/Cgl/wiki/CglProbing
[CglSimpleRounding]: https://github.com/coin-or/Cgl/wiki/CglSimpleRounding