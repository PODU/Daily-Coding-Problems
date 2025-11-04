// Count columns that are NOT non-decreasing top-to-bottom; that's the min to remove.
// O(N*M) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

int minDeletions(const vector<string>& grid) {
    if (grid.empty()) return 0;
    int rows = grid.size(), cols = grid[0].size(), count = 0;
    for (int c = 0; c < cols; c++) {
        for (int r = 1; r < rows; r++) {
            if (grid[r][c] < grid[r - 1][c]) { count++; break; }
        }
    }
    return count;
}

int main() {
    cout << minDeletions({"cba", "daf", "ghi"}) << "\n"; // 1
    cout << minDeletions({"abcdef"}) << "\n";            // 0
    cout << minDeletions({"zyx", "wvu", "tsr"}) << "\n"; // 3
    return 0;
}
