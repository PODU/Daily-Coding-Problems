// Count columns that are NOT non-decreasing top-to-bottom; that's the min to remove.
// O(N*M) time, O(1) extra space.

function minDeletions(grid) {
    if (grid.length === 0) return 0;
    const rows = grid.length, cols = grid[0].length;
    let count = 0;
    for (let c = 0; c < cols; c++) {
        for (let r = 1; r < rows; r++) {
            if (grid[r][c] < grid[r - 1][c]) { count++; break; }
        }
    }
    return count;
}

function main() {
    console.log(minDeletions(["cba", "daf", "ghi"])); // 1
    console.log(minDeletions(["abcdef"]));            // 0
    console.log(minDeletions(["zyx", "wvu", "tsr"])); // 3
}

main();
