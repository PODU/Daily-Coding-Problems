// Min perfect squares: Legendre/Lagrange three-square theorem gives the count in
// O(sqrt N); decomposition found greedily largest-square-first. Time O(sqrt N), Space O(1).
"use strict";

function isSquare(n) {
    const r = Math.floor(Math.sqrt(n));
    return r * r === n || (r + 1) * (r + 1) === n;
}

function minSquaresCount(n) {
    if (isSquare(n)) return 1;
    for (let a = 1; a * a <= n; a++)
        if (isSquare(n - a * a)) return 2;
    let m = n;
    while (m % 4 === 0) m /= 4;
    if (m % 8 === 7) return 4;
    return 3;
}

function find(n, count, out) {
    if (count === 1) {
        if (isSquare(n)) { out.push(n); return true; }
        return false;
    }
    for (let a = Math.floor(Math.sqrt(n)) + 1; a >= 1; a--) {
        if (a * a > n) continue;
        if (find(n - a * a, count - 1, out)) { out.push(a * a); return true; }
    }
    return false;
}

function demo(n) {
    const count = minSquaresCount(n);
    const parts = [];
    find(n, count, parts);
    parts.sort((a, b) => b - a);
    console.log(`${count} (${parts.join(" + ")})`);
}

demo(4);
demo(17);
demo(18);
