// All permutations via backtracking, picking remaining elements left-to-right (lexicographic order).
// Time O(n! * n), Space O(n) recursion + output.
#include <bits/stdc++.h>
using namespace std;

void backtrack(vector<int>& nums, vector<bool>& used, vector<int>& cur,
               vector<vector<int>>& res) {
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

vector<vector<int>> permute(vector<int> nums) {
    vector<vector<int>> res;
    vector<bool> used(nums.size(), false);
    vector<int> cur;
    backtrack(nums, used, cur, res);
    return res;
}

int main() {
    auto res = permute({1, 2, 3});
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
