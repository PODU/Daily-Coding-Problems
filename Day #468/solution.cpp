// Rotate NxN matrix 90 deg clockwise in place: transpose then reverse each row.
// Time: O(n^2), Space: O(1).
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
    vector<vector<int>> m = {{1,2,3},{4,5,6},{7,8,9}};
    rotate(m);
    int n = m.size();
    for (int i = 0; i < n; i++) {
        cout << (i == 0 ? "[[" : " [");
        for (int j = 0; j < n; j++) {
            cout << m[i][j];
            if (j < n - 1) cout << ", ";
        }
        cout << "]";
        if (i < n - 1) cout << ",\n";
        else cout << "]\n";
    }
    return 0;
}
