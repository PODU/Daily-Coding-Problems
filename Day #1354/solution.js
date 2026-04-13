// Minimum jumps to reach end. Greedy: track current reach, farthest reach, jumps. O(N) time, O(1) space.

function minJumps(a) {
    const n = a.length;
    if (n <= 1) return 0;
    let jumps = 0, curEnd = 0, farthest = 0;
    for (let i = 0; i < n - 1; i++) {
        farthest = Math.max(farthest, i + a[i]);
        if (i === curEnd) {
            jumps++;
            curEnd = farthest;
            if (curEnd >= n - 1) break;
        }
    }
    return jumps;
}

const a = [6, 2, 4, 0, 5, 1, 1, 4, 2, 9];
console.log(minJumps(a));
