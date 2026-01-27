// Day 967: Find the duplicate in array of n+1 ints from {1..n}.
// Approach: Floyd's tortoise & hare on value->index cycle. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int findDuplicate(const vector<int>& nums) {
    int slow = nums[0], fast = nums[0];
    do {
        slow = nums[slow];
        fast = nums[nums[fast]];
    } while (slow != fast);
    slow = nums[0];
    while (slow != fast) { slow = nums[slow]; fast = nums[fast]; }
    return slow;
}

int main() {
    cout << findDuplicate({1, 3, 4, 2, 2}) << endl; // 2
    return 0;
}
