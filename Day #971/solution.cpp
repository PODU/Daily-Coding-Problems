// Day 971: Rotate N x N matrix 90 degrees clockwise in place.
// Approach: transpose then reverse each row. Time O(N^2), Space O(1).
#include <bits/stdc++.h>
using namespace std;

void rotate(vector<vector<int>>& m) {
    int n = m.size();
    for (int i = 0; i < n; ++i)
        for (int j = i + 1; j < n; ++j)
            swap(m[i][j], m[j][i]);
    for (int i = 0; i < n; ++i)
        reverse(m[i].begin(), m[i].end());
}

int main() {
    vector<vector<int>> m = {{1,2,3},{4,5,6},{7,8,9}};
    rotate(m);
    for (auto& row : m) {
        for (int j = 0; j < (int)row.size(); ++j)
            cout << row[j] << (j + 1 < (int)row.size() ? " " : "");
        cout << "\n";
    }
    // 7 4 1 / 8 5 2 / 9 6 3
    return 0;
}
