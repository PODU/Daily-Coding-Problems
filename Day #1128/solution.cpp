// All permutations of a list of digits in lexicographic order.
// Backtracking over sorted digits. O(n!*n) time, O(n) extra space.
#include <bits/stdc++.h>
using namespace std;

void backtrack(vector<int>& digits, vector<bool>& used, vector<int>& cur, vector<vector<int>>& res) {
    if (cur.size() == digits.size()) { res.push_back(cur); return; }
    for (int i = 0; i < (int)digits.size(); i++) {
        if (used[i]) continue;
        used[i] = true;
        cur.push_back(digits[i]);
        backtrack(digits, used, cur, res);
        cur.pop_back();
        used[i] = false;
    }
}

int main() {
    vector<int> digits = {1, 2, 3};
    sort(digits.begin(), digits.end());
    vector<bool> used(digits.size(), false);
    vector<int> cur;
    vector<vector<int>> res;
    backtrack(digits, used, cur, res);

    cout << "[";
    for (size_t i = 0; i < res.size(); i++) {
        cout << "[";
        for (size_t j = 0; j < res[i].size(); j++) {
            cout << res[i][j];
            if (j + 1 < res[i].size()) cout << ",";
        }
        cout << "]";
        if (i + 1 < res.size()) cout << ",";
    }
    cout << "]" << endl;
    return 0;
}
