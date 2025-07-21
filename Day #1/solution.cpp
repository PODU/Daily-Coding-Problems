// Two-sum existence: one pass with a hash set of complements.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

bool twoSum(const vector<int>& nums, int k) {
    unordered_set<int> seen;
    for (int x : nums) {
        if (seen.count(k - x)) return true;
        seen.insert(x);
    }
    return false;
}

int main() {
    vector<int> nums = {10, 15, 3, 7};
    int k = 17;
    cout << (twoSum(nums, k) ? "true" : "false") << "\n";
    return 0;
}
