// Day 1269: Rotate N x N matrix 90 degrees clockwise, in place.
// Transpose then reverse each row. O(n^2) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

void rotate(vector<vector<int>>& m) {
    int n = m.size();
    for (int i = 0; i < n; ++i)
        for (int j = i + 1; j < n; ++j) swap(m[i][j], m[j][i]);
    for (auto& row : m) reverse(row.begin(), row.end());
}

int main() {
    vector<vector<int>> m = {{1,2,3},{4,5,6},{7,8,9}};
    rotate(m);
    cout << "[";
    for (size_t i = 0; i < m.size(); ++i) {
        cout << "[";
        for (size_t j = 0; j < m[i].size(); ++j) { cout << m[i][j]; if (j + 1 < m[i].size()) cout << ", "; }
        cout << "]";
        if (i + 1 < m.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
