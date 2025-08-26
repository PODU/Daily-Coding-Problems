// Monte Carlo simulation of two dice stopping games; average rolls. O(trials) time, O(1) space.
// Theory: E[rolls "5 then 6"]=36, E[rolls "5 then 5"]=42. Exact sim values depend on RNG/seed.
let state = 12345 >>> 0; // seeded LCG for reproducibility
// Use the LCG's high bits: its low bits are correlated mod 6 (a and c are both
// 3 mod 6), which makes a 5 ever being followed by a 5 or 6 impossible.
function die() { state = (Math.imul(state, 1103515245) + 12345) >>> 0; return ((state >>> 16) % 6) + 1; }

function trial(second) {
    let rolls = 0, prev = 0;
    while (true) { const c = die(); rolls++; if (prev === 5 && c === second) return rolls; prev = c; }
}

function main() {
    const T = 100000;
    let s1 = 0, s2 = 0;
    for (let i = 0; i < T; i++) s1 += trial(6);
    for (let i = 0; i < T; i++) s2 += trial(5);
    const e1 = s1 / T, e2 = s2 / T;
    console.log(`Game 1 (five then six) expected rolls: ${e1.toFixed(2)}`);
    console.log(`Game 2 (five then five) expected rolls: ${e2.toFixed(2)}`);
    console.log(e1 < e2
        ? "Alice should play Game 1 (five then six), it has lower expected cost."
        : "Alice should play Game 2 (five then five), it has lower expected cost.");
}

main();
