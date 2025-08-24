// Rotate NxN 90 clockwise in place: transpose then reverse each row. O(n^2) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

void rotate(vector<vector<int>>& m) {
    int n = m.size();
    for (int i = 0; i < n; i++)
        for (int j = i + 1; j < n; j++)
            swap(m[i][j], m[j][i]);
    for (int i = 0; i < n; i++)
        reverse(m[i].begin(), m[i].end());
}

int main() {
    vector<vector<int>> m = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
    rotate(m);
    for (auto& row : m) {
        cout << "[";
        for (size_t j = 0; j < row.size(); j++) {
            if (j) cout << ", ";
            cout << row[j];
        }
        cout << "]" << endl;
    }
    return 0;
}
