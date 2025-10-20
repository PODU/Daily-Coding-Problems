// Day 463: Minimum Quxes remaining after merges.
// Approach: if only one color present, none can merge -> total count. Else if all three
// color counts share the same parity -> 2, otherwise -> 1. Time: O(n), Space: O(1).
"use strict";

function minQuxes(quxes) {
  let r = 0, g = 0, b = 0;
  for (const c of quxes) {
    if (c === "R") r++;
    else if (c === "G") g++;
    else b++;
  }
  const present = (r > 0) + (g > 0) + (b > 0);
  if (present <= 1) return r + g + b; // all same color (or empty)
  if (r % 2 === g % 2 && g % 2 === b % 2) return 2;
  return 1;
}

const quxes = ["R", "G", "B", "G", "B"];
console.log(minQuxes(quxes));
