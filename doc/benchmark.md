# Benchmark

## Idea to optimize

1. Replace `CellPosition` with `IVec2` from bevy_math
2. Use SIMD for vector operations
3. Optimize vector allocations
4. Improve datas locality with a better layout
5. Use rayon !

## Description
This file repertored some benchmark I've made. That's not a professional benchmark. The process is, run `btop` in a shell, run `cargo run --release` in an other shell and observe.

The following hash after the word **benchmark** is the commit hash were I've done the benchmark


### Benchmark 6531b07

Random generation on 1000 * 1000
 
Desktop environnement :

FPS: 9.34
FPS: 11.05
FPS: 12.51
FPS: 13.53
FPS: 15.46

Web environment :
