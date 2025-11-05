// Non-decreasing with at most one modification: single pass, greedy fix. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool checkPossibility(vector<int> nums) {
    int cnt = 0;
    for (size_t i = 1; i < nums.size(); ++i) {
        if (nums[i] < nums[i - 1]) {
            if (++cnt > 1) return false;
            if (i < 2 || nums[i - 2] <= nums[i]) nums[i - 1] = nums[i];
            else nums[i] = nums[i - 1];
        }
    }
    return true;
}

int main() {
    cout << (checkPossibility({10, 5, 7}) ? "true" : "false") << "\n";
    cout << (checkPossibility({10, 5, 1}) ? "true" : "false") << "\n";
    return 0;
}
