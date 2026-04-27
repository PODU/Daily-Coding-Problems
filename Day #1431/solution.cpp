// Day 1431: Majority element (appears > floor(n/2)).
// Approach: Boyer-Moore voting. Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int majorityElement(const vector<int>& nums) {
    int count = 0, candidate = 0;
    for (int x : nums) {
        if (count == 0) candidate = x;
        count += (x == candidate) ? 1 : -1;
    }
    return candidate;
}

int main() {
    vector<int> nums = {1, 2, 1, 1, 3, 4, 0};
    cout << majorityElement(nums) << endl; // 1
    return 0;
}
