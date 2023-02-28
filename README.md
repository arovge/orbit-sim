# orbit-sim

A simple orbit simulator

This is inspired by the gold standard of orbital simulations: Orbit-sim, an orbital simulator created by the notorious 100x unicorn dev "Nettcod": https://github.com/Nettcod/Orbit-Sim

### Usage

To run the program, simply clone the repository and run:

```bash
cargo run
```

### Controls

* Simulating the asteroid in gravity
    * Left click to drop the asteroid
    * Left click and hold to give the asteroid an initial velocity
* Edit planets
    * Press `i` and verify the bottom left corner says `Edit planets`
    * Left click to place a planet
    * Right click over a placed planet to remove it
    * Press `i` again to exit the edit planets mode
        * You'll then see `Following cursor` in the bottom left
* Reset asteroid back to following the cursor
    * Press `R`
* Exit simulation
    * Press `Esc` or `Q`
