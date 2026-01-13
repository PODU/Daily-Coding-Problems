// Paint houses: DP tracking two smallest costs of previous row -> O(N*K) time, O(1) extra space.
// For each house we only need the min and second-min of the previous row to avoid same color.

function minCost(costs) {
    if (costs.length === 0) return 0;
    let prevMin = 0, prevSecond = 0, prevIdx = -1;
    for (const row of costs) {
        let curMin = Infinity, curSecond = Infinity, curIdx = -1;
        for (let c = 0; c < row.length; c++) {
            const cost = row[c] + (c === prevIdx ? prevSecond : prevMin);
            if (cost < curMin) {
                curSecond = curMin;
                curMin = cost;
                curIdx = c;
            } else if (cost < curSecond) {
                curSecond = cost;
            }
        }
        prevMin = curMin;
        prevSecond = curSecond;
        prevIdx = curIdx;
    }
    return prevMin;
}

function main() {
    const costs = [[1, 5, 3], [2, 9, 4]];
    console.log(minCost(costs));
}

main();
