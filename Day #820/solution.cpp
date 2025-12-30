// First missing positive via cyclic sort: place nums[i] at index nums[i]-1, then scan. O(n) time, O(1) space.
#include <iostream>
#include <vector>
using namespace std;

int firstMissingPositive(vector<int> nums) {
    int n = nums.size();
    for (int i = 0; i < n; i++) {
        while (nums[i] > 0 && nums[i] <= n && nums[nums[i] - 1] != nums[i])
            swap(nums[i], nums[nums[i] - 1]);
    }
    for (int i = 0; i < n; i++)
        if (nums[i] != i + 1) return i + 1;
    return n + 1;
}

int main() {
    cout << firstMissingPositive({3, 4, -1, 1}) << "\n";
    cout << firstMissingPositive({1, 2, 0}) << "\n";
    return 0;
}
