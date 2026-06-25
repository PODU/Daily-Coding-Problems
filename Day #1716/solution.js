// Day 1716: Print an N x M matrix in clockwise spiral order.
// Boundary-shrinking traversal. Time: O(N*M), Space: O(1) extra.

function spiral(m) {
    if (m.length === 0 || m[0].length === 0) return;
    let top = 0, bottom = m.length - 1;
    let left = 0, right = m[0].length - 1;
    const out = [];
    while (top <= bottom && left <= right) {
        for (let c = left; c <= right; c++) out.push(m[top][c]);
        top++;
        for (let r = top; r <= bottom; r++) out.push(m[r][right]);
        right--;
        if (top <= bottom) {
            for (let c = right; c >= left; c--) out.push(m[bottom][c]);
            bottom--;
        }
        if (left <= right) {
            for (let r = bottom; r >= top; r--) out.push(m[r][left]);
            left++;
        }
    }
    console.log(out.join("\n"));
}

const matrix = [
    [1, 2, 3, 4, 5],
    [6, 7, 8, 9, 10],
    [11, 12, 13, 14, 15],
    [16, 17, 18, 19, 20],
];
spiral(matrix);
