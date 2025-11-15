// Day 600: Closest pair of points on a plane.
// Approach: divide & conquer with a y-sorted strip check. Time O(n log n), Space O(n).
'use strict';

function dist2(a, b) {
    const dx = a[0] - b[0], dy = a[1] - b[1];
    return dx * dx + dy * dy;
}

function closest(px) {
    function rec(lo, hi) {
        const n = hi - lo;
        if (n <= 3) {
            let best = Infinity, bp = null;
            for (let i = lo; i < hi; i++)
                for (let j = i + 1; j < hi; j++) {
                    const d = dist2(px[i], px[j]);
                    if (d < best) { best = d; bp = [px[i], px[j]]; }
                }
            return [best, bp];
        }
        const mid = (lo + hi) >> 1;
        const midx = px[mid][0];
        const [bl, pl] = rec(lo, mid);
        const [br, pr] = rec(mid, hi);
        let best, bp;
        if (bl <= br) { best = bl; bp = pl; } else { best = br; bp = pr; }
        let dd = Math.sqrt(best);
        const strip = [];
        for (let i = lo; i < hi; i++)
            if (Math.abs(px[i][0] - midx) < dd) strip.push(px[i]);
        strip.sort((a, b) => a[1] - b[1]);
        for (let i = 0; i < strip.length; i++)
            for (let j = i + 1; j < strip.length && (strip[j][1] - strip[i][1]) < dd; j++) {
                const d = dist2(strip[i], strip[j]);
                if (d < best) { best = d; bp = [strip[i], strip[j]]; dd = Math.sqrt(best); }
            }
        return [best, bp];
    }
    return rec(0, px.length);
}

function main() {
    const pts = [[1, 1], [-1, -1], [3, 4], [6, 1], [-1, -6], [-4, -3]];
    pts.sort((a, b) => a[0] !== b[0] ? a[0] - b[0] : a[1] - b[1]);
    let [, [a, b]] = closest(pts);
    if (a[0] > b[0] || (a[0] === b[0] && a[1] > b[1])) { const t = a; a = b; b = t; }
    console.log(`[(${a[0]}, ${a[1]}), (${b[0]}, ${b[1]})]`);
}

main();
