// Approach: Pairwise O(n^2) overlap check on axis-aligned rectangles (strict, positive-area).
// Time O(n^2), Space O(1).
'use strict';

// top_left (x,y), dims (w,h): x in [x,x+w], y in [y-h,y]
function make(x, y, w, h) { return { x1: x, y1: y - h, x2: x + w, y2: y }; }

function overlap(a, b) {
    return a.x1 < b.x2 && b.x1 < a.x2 && a.y1 < b.y2 && b.y1 < a.y2;
}

function anyOverlap(rs) {
    for (let i = 0; i < rs.length; i++)
        for (let j = i + 1; j < rs.length; j++)
            if (overlap(rs[i], rs[j])) return true;
    return false;
}

const rs = [make(1, 4, 3, 3), make(-1, 3, 2, 1), make(0, 5, 4, 3)];
console.log(anyOverlap(rs) ? "true" : "false");
