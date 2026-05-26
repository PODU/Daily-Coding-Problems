// Staircase count from top-right in O(N+M): smaller = count(<low); larger = N*M - count(<high). Time O(N+M), Space O(1).
#include <bits/stdc++.h>
using namespace std;

// Number of elements strictly less than x in a row/col sorted matrix (staircase walk).
long long countLess(const vector<vector<int>>& M, int x) {
    int n = M.size(), m = M[0].size();
    long long cnt = 0;
    int r = 0, c = m - 1;
    while (r < n && c >= 0) {
        if (M[r][c] < x) { cnt += c + 1; r++; }
        else c--;
    }
    return cnt;
}

int main() {
    vector<vector<int>> M = {
        {1, 3, 7, 10, 15, 20},
        {2, 6, 9, 14, 22, 25},
        {3, 8, 10, 15, 25, 30},
        {10, 11, 12, 23, 30, 35},
        {20, 25, 30, 35, 40, 45}
    };
    int low = M[1][1];   // 6
    int high = M[3][3];  // 23
    long long total = (long long)M.size() * M[0].size();
    long long smaller = countLess(M, low);          // elements < 6
    long long larger  = total - countLess(M, high); // elements >= 23
    cout << (smaller + larger) << endl;
    return 0;
}
