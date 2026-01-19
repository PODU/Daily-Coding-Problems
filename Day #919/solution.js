// Brick wall: count edge offsets (cumulative sums excluding last) via hashmap.
// Fewest cuts = numRows - maxAlignedEdges. Time O(total bricks), Space O(distinct edges).
function leastBricks(wall) {
    const freq = new Map();
    let best = 0;
    for (const row of wall) {
        let sum = 0;
        for (let i = 0; i + 1 < row.length; i++) {
            sum += row[i];
            const c = (freq.get(sum) || 0) + 1;
            freq.set(sum, c);
            if (c > best) best = c;
        }
    }
    return wall.length - best;
}

function main() {
    const wall = [[3,5,1,1],[2,3,3,2],[5,5],[4,4,2],[1,3,3,3],[1,1,6,1,1]];
    console.log(leastBricks(wall));
}

main();
