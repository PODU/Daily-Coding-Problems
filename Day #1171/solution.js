// Day 1171: Validate an American-style crossword grid.
// Checks rotational symmetry, white-square connectivity (BFS), and that every
// maximal horizontal/vertical white run has length >= 3.
// Time O(N^2), Space O(N^2).  '#' = black, '.' = white.

function isValidCrossword(g) {
    const n = g.length;
    if (n === 0 || g.some(r => r.length !== n)) return false;

    // 1. Rotational symmetry.
    for (let i = 0; i < n; i++)
        for (let j = 0; j < n; j++)
            if ((g[i][j] === '.') !== (g[n-1-i][n-1-j] === '.')) return false;

    // 2 & 3. White runs (rows and columns) must be >= 3.
    const runsOk = (get) => {
        for (let a = 0; a < n; a++) {
            let run = 0;
            for (let b = 0; b <= n; b++) {
                if (b < n && get(a, b) === '.') run++;
                else { if (run > 0 && run < 3) return false; run = 0; }
            }
        }
        return true;
    };
    if (!runsOk((i, j) => g[i][j])) return false;
    if (!runsOk((j, i) => g[i][j])) return false;

    // 4. Connectivity.
    const whites = [];
    for (let i = 0; i < n; i++)
        for (let j = 0; j < n; j++)
            if (g[i][j] === '.') whites.push([i, j]);
    if (whites.length === 0) return true;
    const key = (x, y) => x * n + y;
    const seen = new Set([key(...whites[0])]);
    const q = [whites[0]];
    while (q.length) {
        const [x, y] = q.pop();
        for (const [nx, ny] of [[x+1,y],[x-1,y],[x,y+1],[x,y-1]]) {
            if (nx >= 0 && nx < n && ny >= 0 && ny < n && g[nx][ny] === '.' && !seen.has(key(nx, ny))) {
                seen.add(key(nx, ny));
                q.push([nx, ny]);
            }
        }
    }
    return seen.size === whites.length;
}

const g1 = [".....", ".....", ".....", ".....", "....."];
const g2 = [".#...", ".....", ".....", ".....", "...#."];
console.log(isValidCrossword(g1) ? "true" : "false");
console.log(isValidCrossword(g2) ? "true" : "false");
