// Day 1747: All permutations of a list of digits.
// Approach: backtracking with a used[] mask, iterating values in order -> lexicographic.
// Time O(n * n!), space O(n) recursion (plus O(n!) for the output).
#include <bits/stdc++.h>
using namespace std;

void backtrack(const vector<int>& nums, vector<bool>& used,
               vector<int>& cur, vector<vector<int>>& res) {
    if (cur.size() == nums.size()) { res.push_back(cur); return; }
    for (size_t i = 0; i < nums.size(); ++i) {
        if (used[i]) continue;
        used[i] = true;
        cur.push_back(nums[i]);
        backtrack(nums, used, cur, res);
        cur.pop_back();
        used[i] = false;
    }
}

vector<vector<int>> permutations(vector<int> nums) {
    vector<vector<int>> res;
    vector<int> cur;
    vector<bool> used(nums.size(), false);
    backtrack(nums, used, cur, res);
    return res;
}

int main() {
    vector<int> nums = {1, 2, 3};
    auto res = permutations(nums);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << "[";
        for (size_t j = 0; j < res[i].size(); ++j) {
            cout << res[i][j];
            if (j + 1 < res[i].size()) cout << ",";
        }
        cout << "]";
        if (i + 1 < res.size()) cout << ",";
    }
    cout << "]\n";
    return 0;
}
