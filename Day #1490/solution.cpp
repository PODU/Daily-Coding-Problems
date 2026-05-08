// Find the duplicate in array of length n+1 with values in {1..n}.
// Floyd's tortoise-and-hare cycle detection. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int findDuplicate(const vector<int>& nums) {
    int slow = nums[0], fast = nums[0];
    do {
        slow = nums[slow];
        fast = nums[nums[fast]];
    } while (slow != fast);
    slow = nums[0];
    while (slow != fast) {
        slow = nums[slow];
        fast = nums[fast];
    }
    return slow;
}

int main() {
    vector<int> nums = {1, 2, 3, 4, 2}; // n = 4
    cout << findDuplicate(nums) << endl; // expected: 2
    return 0;
}
