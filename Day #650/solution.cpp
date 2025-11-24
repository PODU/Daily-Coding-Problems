// Per-row binary search: count elements < A[i1][j1] (lower_bound) plus elements >= A[i2][j2] (M - lower_bound).
// Upper bound taken inclusive (>=) to match reference example. Time O(N log M), space O(1).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<vector<int>> A = {
        {1, 3, 7, 10, 15, 20},
        {2, 6, 9, 14, 22, 25},
        {3, 8, 10, 15, 25, 30},
        {10, 11, 12, 23, 30, 35},
        {20, 25, 30, 35, 40, 45}
    };
    int i1 = 1, j1 = 1, i2 = 3, j2 = 3;
    int pivot1 = A[i1][j1]; // 6
    int pivot2 = A[i2][j2]; // 23
    int M = A[0].size();
    long long count_smaller = 0, count_upper = 0;
    for (auto &row : A) {
        // strictly less than pivot1
        count_smaller += lower_bound(row.begin(), row.end(), pivot1) - row.begin();
        // >= pivot2 (inclusive upper bound)
        count_upper += M - (lower_bound(row.begin(), row.end(), pivot2) - row.begin());
    }
    cout << (count_smaller + count_upper) << "\n";
    return 0;
}
