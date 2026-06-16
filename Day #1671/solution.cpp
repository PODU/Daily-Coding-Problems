// Day 1671: Min columns to remove so each column is non-decreasing top->bottom.
// Count columns containing any out-of-order adjacent pair. Time O(N*M), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minDeletions(const vector<string>& g) {
    if (g.empty()) return 0;
    int rows = g.size(), cols = g[0].size(), del = 0;
    for (int j = 0; j < cols; ++j)
        for (int i = 0; i + 1 < rows; ++i)
            if (g[i][j] > g[i + 1][j]) { ++del; break; }
    return del;
}

int main() {
    vector<string> grid = {"cba", "daf", "ghi"};
    cout << minDeletions(grid) << "\n"; // 1
    return 0;
}
