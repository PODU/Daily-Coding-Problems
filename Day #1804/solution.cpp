// Day 1804: Find a duplicate in array of n+1 elements from {1..n} using a seen[] array.
// O(n) time, O(n) space.
#include <iostream>
#include <vector>

int findDuplicate(const std::vector<int>& nums) {
    std::vector<bool> seen(nums.size() + 1, false);
    for (int x : nums) {
        if (seen[x]) return x;
        seen[x] = true;
    }
    return -1; // no duplicate (won't happen per problem constraints)
}

int main() {
    std::vector<int> nums = {1, 3, 4, 2, 2};
    std::cout << findDuplicate(nums) << "\n"; // expected 2
    return 0;
}
