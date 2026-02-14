// Sliding window with count map: longest subarray with at most 2 distinct values. O(n) time, O(1) space.

function longestAtMost2Distinct(nums) {
    const cnt = new Map();
    let left = 0, best = 0;
    for (let right = 0; right < nums.length; right++) {
        cnt.set(nums[right], (cnt.get(nums[right]) || 0) + 1);
        while (cnt.size > 2) {
            const l = nums[left];
            cnt.set(l, cnt.get(l) - 1);
            if (cnt.get(l) === 0) cnt.delete(l);
            left++;
        }
        best = Math.max(best, right - left + 1);
    }
    return best;
}

const nums = [2,1,2,3,3,1,3,5];
console.log(longestAtMost2Distinct(nums));
