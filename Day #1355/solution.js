// Longest run of consecutive 1s in binary. Bit trick: n &= (n<<1) collapses runs; iterations = longest run. O(run) time, O(1) space.

function longestRun(n) {
    let count = 0;
    while (n !== 0) {
        n &= (n << 1);
        count++;
    }
    return count;
}

console.log(longestRun(156));
