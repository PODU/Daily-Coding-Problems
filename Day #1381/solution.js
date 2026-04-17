// Conway's Game of Life on an infinite board using a set of live coordinates.
// Each step counts neighbours only around live cells.
// Time: O(L) per step, Space: O(L).
"use strict";

function key(x, y) { return x + "," + y; }
function parse(k) { return k.split(",").map(Number); }

function step(live) {
    const cnt = new Map();
    for (const k of live) {
        const [x, y] = parse(k);
        for (let dx = -1; dx <= 1; dx++)
            for (let dy = -1; dy <= 1; dy++)
                if (dx || dy) {
                    const nk = key(x + dx, y + dy);
                    cnt.set(nk, (cnt.get(nk) || 0) + 1);
                }
    }
    const nb = new Set();
    for (const [k, c] of cnt)
        if (c === 3 || (c === 2 && live.has(k))) nb.add(k);
    return nb;
}

function render(live) {
    if (live.size === 0) { console.log("(empty)"); return; }
    let minx = Infinity, maxx = -Infinity, miny = Infinity, maxy = -Infinity;
    for (const k of live) {
        const [x, y] = parse(k);
        minx = Math.min(minx, x); maxx = Math.max(maxx, x);
        miny = Math.min(miny, y); maxy = Math.max(maxy, y);
    }
    for (let x = minx; x <= maxx; x++) {
        let row = "";
        for (let y = miny; y <= maxy; y++) row += live.has(key(x, y)) ? "*" : ".";
        console.log(row);
    }
}

function run(cells, steps) {
    let live = new Set(cells.map(([x, y]) => key(x, y)));
    for (let i = 0; i <= steps; i++) {
        console.log("Step " + i + ":");
        render(live);
        console.log();
        live = step(live);
    }
}

run([[1, 0], [1, 1], [1, 2]], 2); // blinker
