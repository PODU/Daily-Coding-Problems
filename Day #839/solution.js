// Day 839: Max number of dictionary words packed on an NxN board.
// For each word collect all valid adjacent-path placements (DFS), then backtrack
// over words choosing a disjoint set to maximize the count.
// Time O(exponential worst-case) on small boards; placement search bounded by board size.
'use strict';

let R, C;

function findPlacements(mat, word) {
    const placements = new Set();
    function dfs(r, c, idx, used) {
        if (mat[r][c] !== word[idx]) return;
        used |= (1 << (r * C + c));
        if (idx === word.length - 1) {
            placements.add(used);
            return;
        }
        const dr = [1, -1, 0, 0], dc = [0, 0, 1, -1];
        for (let d = 0; d < 4; d++) {
            const nr = r + dr[d], nc = c + dc[d];
            if (nr >= 0 && nr < R && nc >= 0 && nc < C && !(used & (1 << (nr * C + nc))))
                dfs(nr, nc, idx + 1, used);
        }
    }
    for (let i = 0; i < R; i++)
        for (let j = 0; j < C; j++)
            dfs(i, j, 0, 0);
    return [...placements];
}

function maxWords(mat, dict) {
    const wordPlacements = [];
    for (const w of dict) {
        const p = findPlacements(mat, w);
        if (p.length) wordPlacements.push(p);
    }
    let best = 0;
    function backtrack(i, occupied, count) {
        best = Math.max(best, count);
        if (i === wordPlacements.length) return;
        backtrack(i + 1, occupied, count); // skip
        for (const tiles of wordPlacements[i])
            if (!(occupied & tiles))
                backtrack(i + 1, occupied | tiles, count + 1);
    }
    backtrack(0, 0, 0);
    return best;
}

const mat = [
    ['e', 'a', 'n'],
    ['t', 't', 'i'],
    ['a', 'r', 'a'],
];
R = mat.length;
C = mat[0].length;
const dict = ['eat', 'rain', 'in', 'rat'];
console.log(maxWords(mat, dict));
