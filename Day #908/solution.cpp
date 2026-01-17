// Count columns to delete so each remaining column is non-decreasing top->bottom.
// Scan each column for any adjacent out-of-order pair. Time O(N*M), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minDeletions(const vector<string>& grid) {
    if (grid.empty()) return 0;
    int n = grid.size(), m = grid[0].size(), count = 0;
    for (int c = 0; c < m; c++)
        for (int i = 0; i + 1 < n; i++)
            if (grid[i][c] > grid[i + 1][c]) { count++; break; }
    return count;
}

int main() {
    vector<string> grid = {"cba", "daf", "ghi"};
    cout << minDeletions(grid) << endl;
    return 0;
}
