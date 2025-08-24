// Rotate NxN 90 clockwise in place: transpose then reverse each row. O(n^2) time, O(1) extra space.

function rotate(m) {
    const n = m.length;
    for (let i = 0; i < n; i++)
        for (let j = i + 1; j < n; j++)
            [m[i][j], m[j][i]] = [m[j][i], m[i][j]];
    for (let i = 0; i < n; i++) m[i].reverse();
}

function main() {
    const m = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    rotate(m);
    for (const row of m) console.log("[" + row.join(", ") + "]");
}

main();
