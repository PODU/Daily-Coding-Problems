// Day 96: All permutations via backtracking on the remaining elements, yielding
// lexicographic order. O(n*n!) time.
#include <bits/stdc++.h>
using namespace std;

void backtrack(vector<int>& path, vector<int> rem, vector<vector<int>>& res) {
    if (rem.empty()) { res.push_back(path); return; }
    for (size_t i = 0; i < rem.size(); i++) {
        path.push_back(rem[i]);
        vector<int> next = rem;
        next.erase(next.begin() + i);
        backtrack(path, next, res);
        path.pop_back();
    }
}

int main() {
    vector<int> nums = {1, 2, 3};
    vector<vector<int>> res;
    vector<int> path;
    backtrack(path, nums, res);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) {
        cout << "[";
        for (size_t j = 0; j < res[i].size(); j++)
            cout << res[i][j] << (j + 1 < res[i].size() ? ", " : "");
        cout << "]" << (i + 1 < res.size() ? ", " : "");
    }
    cout << "]\n";
    return 0;
}
