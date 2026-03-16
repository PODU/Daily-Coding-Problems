// Day 1216: Min columns to delete so each column is non-decreasing top->bottom.
// Approach: scan each column once, count unsorted columns. Time O(N*M), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minDeletions(const vector<string>& grid) {
    if (grid.empty()) return 0;
    int rows = grid.size(), cols = grid[0].size(), count = 0;
    for (int c = 0; c < cols; ++c)
        for (int r = 1; r < rows; ++r)
            if (grid[r][c] < grid[r-1][c]) { ++count; break; }
    return count;
}

int main() {
    vector<string> grid = {"cba", "daf", "ghi"};
    cout << minDeletions(grid) << endl; // 1
    return 0;
}
