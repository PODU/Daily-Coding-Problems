// N-Queens count via bitmask backtracking (columns + two diagonals). O(N!) worst, O(N) space.
#include <iostream>
using namespace std;

int solve(int row, int cols, int diag1, int diag2, int full) {
    if (cols == full) return 1;
    int count = 0;
    int avail = ~(cols | diag1 | diag2) & full;
    while (avail) {
        int bit = avail & (-avail);
        avail -= bit;
        count += solve(row + 1, cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1, full);
    }
    return count;
}

int countNQueens(int n) {
    return solve(0, 0, 0, 0, (1 << n) - 1);
}

int main() {
    cout << countNQueens(8) << endl;
    return 0;
}
