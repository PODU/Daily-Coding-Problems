// Day 1770: Flood fill via BFS, 4-directional. Replace connected same-colored region.
// Time: O(rows*cols), Space: O(rows*cols) for the queue in worst case.
'use strict';

function floodFill(grid, sr, sc, color) {
    const R = grid.length, C = grid[0].length;
    const target = grid[sr][sc];
    if (target === color) return grid;
    const q = [[sr, sc]];
    grid[sr][sc] = color;
    const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];
    while (q.length) {
        const [r, c] = q.shift();
        for (const [dr, dc] of dirs) {
            const nr = r + dr, nc = c + dc;
            if (nr >= 0 && nr < R && nc >= 0 && nc < C && grid[nr][nc] === target) {
                grid[nr][nc] = color;
                q.push([nr, nc]);
            }
        }
    }
    return grid;
}

const grid = [
    ['B', 'B', 'W'],
    ['W', 'W', 'W'],
    ['W', 'W', 'W'],
    ['B', 'B', 'B'],
];
floodFill(grid, 2, 2, 'G');
console.log(grid.map(row => row.join(' ')).join('\n'));
