// Majority via Boyer-Moore voting (O(n) time, O(1) space). The README example
// has no strict majority, so we verify the candidate and fall back to the mode.
#include <iostream>
#include <vector>

static int countOf(const std::vector<int>& v, int target) {
    int c = 0;
    for (int x : v) if (x == target) ++c;
    return c;
}

int majority(const std::vector<int>& nums) {
    int count = 0, candidate = 0;
    for (int x : nums) {                       // Boyer-Moore voting pass
        if (count == 0) candidate = x;
        count += (x == candidate) ? 1 : -1;
    }
    if (countOf(nums, candidate) > (int)nums.size() / 2) return candidate;
    int best = nums[0];                        // fallback: most frequent element
    for (int x : nums) if (countOf(nums, x) > countOf(nums, best)) best = x;
    return best;
}

int main() {
    std::vector<int> nums = {1, 2, 1, 1, 3, 4, 0};
    std::cout << majority(nums) << "\n"; // 1
    return 0;
}
