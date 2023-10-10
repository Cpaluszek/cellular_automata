use bevy::prelude::*;

// [Conway's Game of Life - Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

// === Rules ===
// n for neighbours
// Liging cells:
//      n < 2 || n > 3 - DIES
//      n == 2 || n == 3 - LIVES
// Dead cells:
//      n == 3 - LIVES
