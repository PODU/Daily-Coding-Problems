// Jump Game: greedy tracking farthest reachable index. Time O(N), Space O(1).
function canJump(nums) {
    let farthest = 0;
    for (let i = 0; i < nums.length; i++) {
        if (i > farthest) return false;
        farthest = Math.max(farthest, i + nums[i]);
    }
    return true;
}

console.log(canJump([2, 0, 1, 0]) ? "True" : "False");
console.log(canJump([1, 1, 0, 1]) ? "True" : "False");
