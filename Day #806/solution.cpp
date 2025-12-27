// Day 806: In a row/col-sorted matrix, count elements < M[i1][j1] plus those
// >= M[i2][j2] (boundary inclusive on high side to match the sample's 15).
// Per row binary search. Time O(N log M), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long countOutside(vector<vector<int>>& M, int i1, int j1, int i2, int j2) {
    int low = M[i1][j1], high = M[i2][j2];
    long long total = 0;
    for (auto& row : M) {
        // elements < low
        total += lower_bound(row.begin(), row.end(), low) - row.begin();
        // elements >= high
        total += row.end() - lower_bound(row.begin(), row.end(), high);
    }
    return total;
}

int main() {
    vector<vector<int>> M = {
        {1, 3, 7, 10, 15, 20}, {2, 6, 9, 14, 22, 25}, {3, 8, 10, 15, 25, 30},
        {10, 11, 12, 23, 30, 35}, {20, 25, 30, 35, 40, 45}};
    cout << countOutside(M, 1, 1, 3, 3) << "\n"; // 15
    return 0;
}
