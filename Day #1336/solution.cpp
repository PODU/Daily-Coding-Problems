// Day 1336: Count distinct N-Queens arrangements.
// Backtracking with column/diagonal bitmasks. Time: O(N!) worst, Space: O(N).
#include <bits/stdc++.h>
using namespace std;

int countQueens(int n, int row, int cols, int d1, int d2) {
    if (row == n) return 1;
    int total = 0;
    int avail = ((1 << n) - 1) & ~(cols | d1 | d2);
    while (avail) {
        int bit = avail & (-avail);
        avail -= bit;
        total += countQueens(n, row + 1, cols | bit, (d1 | bit) << 1, (d2 | bit) >> 1);
    }
    return total;
}

int totalNQueens(int n) { return countQueens(n, 0, 0, 0, 0); }

int main() {
    cout << "N=1 -> " << totalNQueens(1) << "\n";
    cout << "N=4 -> " << totalNQueens(4) << "\n";
    cout << "N=8 -> " << totalNQueens(8) << "\n";
    return 0;
}
