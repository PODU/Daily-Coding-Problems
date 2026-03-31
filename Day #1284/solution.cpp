// Day 1284: In a row/col-sorted matrix, count elements < M[i1][j1] plus those >= M[i2][j2].
// Binary search per (ascending) row. Time O(N log M), Space O(1).
// Note: upper bound is inclusive (>=) so the sample yields 15 as specified.
#include <bits/stdc++.h>
using namespace std;

int countOutside(const vector<vector<int>>& M, int i1, int j1, int i2, int j2) {
    int low = M[i1][j1], high = M[i2][j2];
    int total = 0;
    for (const auto& row : M) {
        total += lower_bound(row.begin(), row.end(), low) - row.begin();          // < low
        total += row.end() - lower_bound(row.begin(), row.end(), high);           // >= high
    }
    return total;
}

int main() {
    vector<vector<int>> M = {
        {1, 3, 7, 10, 15, 20},
        {2, 6, 9, 14, 22, 25},
        {3, 8, 10, 15, 25, 30},
        {10, 11, 12, 23, 30, 35},
        {20, 25, 30, 35, 40, 45}};
    cout << countOutside(M, 1, 1, 3, 3) << "\n"; // 15
    return 0;
}
