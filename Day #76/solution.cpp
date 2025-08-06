// Day 76: Minimum columns to remove so every column is sorted top-to-bottom.
// Greedy scan: count columns that are not non-decreasing. Time O(N*M), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minColumnsToRemove(const vector<string>& grid) {
    if (grid.empty()) return 0;
    int rows = grid.size(), cols = grid[0].size();
    int remove = 0;
    for (int c = 0; c < cols; ++c) {
        for (int r = 1; r < rows; ++r) {
            if (grid[r][c] < grid[r - 1][c]) { ++remove; break; }
        }
    }
    return remove;
}

int main() {
    vector<string> grid = {"cba", "daf", "ghi"};
    cout << minColumnsToRemove(grid) << "\n"; // 1
    return 0;
}
