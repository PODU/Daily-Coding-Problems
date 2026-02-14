// Subset sum DP: dp[i][s] = reachable using first i items; reconstruct path. O(n*k) time/space.

function subsetSum(items, k) {
    const n = items.length;
    const dp = Array.from({length: n+1}, () => new Array(k+1).fill(false));
    dp[0][0] = true;
    for (let i = 1; i <= n; i++) {
        for (let s = 0; s <= k; s++) {
            dp[i][s] = dp[i-1][s];
            if (!dp[i][s] && s >= items[i-1])
                dp[i][s] = dp[i-1][s - items[i-1]];
        }
    }
    if (!dp[n][k]) return null;
    const result = [];
    let s = k;
    for (let i = n; i >= 1; i--) {
        if (!dp[i-1][s]) {
            result.push(items[i-1]);
            s -= items[i-1];
        }
    }
    result.reverse();
    return result;
}

const items = [12, 1, 61, 5, 9, 2];
const res = subsetSum(items, 24);
console.log("Subset: [" + res.join(", ") + "]");
console.log("Sum: " + res.reduce((a, b) => a + b, 0));
console.log(subsetSum(items, 1000));
