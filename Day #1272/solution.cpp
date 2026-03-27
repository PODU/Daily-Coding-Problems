// Day 1272: Determine whether a matrix is Toeplitz (each TL->BR diagonal is constant).
// Compare every cell with its upper-left neighbor. O(N*M) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool isToeplitz(const vector<vector<int>>& m) {
    for (size_t i = 1; i < m.size(); ++i)
        for (size_t j = 1; j < m[i].size(); ++j)
            if (m[i][j] != m[i - 1][j - 1]) return false;
    return true;
}

int main() {
    vector<vector<int>> m = {{1,2,3,4,8},{5,1,2,3,4},{4,5,1,2,3},{7,4,5,1,2}};
    cout << (isToeplitz(m) ? "true" : "false") << endl;
    return 0;
}
