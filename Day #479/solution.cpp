// Generate all permutations via backtracking, picking remaining elements in
// index order so output is lexicographic. Time: O(n! * n), Space: O(n) recursion.
#include <iostream>
#include <vector>
using namespace std;

void backtrack(vector<int>& nums, vector<bool>& used, vector<int>& cur,
               vector<vector<int>>& res) {
    if (cur.size() == nums.size()) {
        res.push_back(cur);
        return;
    }
    for (size_t i = 0; i < nums.size(); ++i) {
        if (used[i]) continue;
        used[i] = true;
        cur.push_back(nums[i]);
        backtrack(nums, used, cur, res);
        cur.pop_back();
        used[i] = false;
    }
}

int main() {
    vector<int> nums = {1, 2, 3};
    vector<bool> used(nums.size(), false);
    vector<int> cur;
    vector<vector<int>> res;
    backtrack(nums, used, cur, res);

    cout << '[';
    for (size_t i = 0; i < res.size(); ++i) {
        if (i) cout << ',';
        cout << '[';
        for (size_t j = 0; j < res[i].size(); ++j) {
            if (j) cout << ',';
            cout << res[i][j];
        }
        cout << ']';
    }
    cout << "]\n";
    return 0;
}
