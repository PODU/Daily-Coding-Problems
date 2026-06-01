// Approach: DP min palindrome partition. pal[i][j] table O(n^2), cut[i]=min cuts for prefix,
// then reconstruct one optimal partition. Time O(n^2), Space O(n^2).
'use strict';

function minPalPartition(s) {
    const n = s.length;
    if (n === 0) return [];
    const pal = Array.from({ length: n }, () => new Array(n).fill(false));
    for (let i = 0; i < n; i++) pal[i][i] = true;
    for (let len = 2; len <= n; len++)
        for (let i = 0; i + len - 1 < n; i++) {
            const j = i + len - 1;
            if (s[i] === s[j] && (len === 2 || pal[i + 1][j - 1])) pal[i][j] = true;
        }
    const cut = new Array(n).fill(0);
    const start = new Array(n).fill(0);
    for (let i = 0; i < n; i++) {
        if (pal[0][i]) { cut[i] = 0; start[i] = 0; continue; }
        let best = Infinity, bj = 0;
        for (let j = 1; j <= i; j++)
            if (pal[j][i] && cut[j - 1] + 1 < best) { best = cut[j - 1] + 1; bj = j; }
        cut[i] = best; start[i] = bj;
    }
    const res = [];
    let i = n - 1;
    while (i >= 0) { const j = start[i]; res.push(s.slice(j, i + 1)); i = j - 1; }
    res.reverse();
    return res;
}

function fmt(v) { return "[" + v.map(x => '"' + x + '"').join(", ") + "]"; }

console.log(fmt(minPalPartition("racecarannakayak")));
console.log(fmt(minPalPartition("abc")));
