// Day 939: Print an N x M matrix in clockwise spiral order.
// Shrink four boundaries layer by layer. Time O(N*M), Space O(1) extra.
#include <bits/stdc++.h>
using namespace std;

void spiral(const vector<vector<int>>& m) {
    if (m.empty()) return;
    int top = 0, bottom = (int)m.size() - 1;
    int left = 0, right = (int)m[0].size() - 1;
    while (top <= bottom && left <= right) {
        for (int c = left; c <= right; ++c) cout << m[top][c] << "\n";
        top++;
        for (int r = top; r <= bottom; ++r) cout << m[r][right] << "\n";
        right--;
        if (top <= bottom) {
            for (int c = right; c >= left; --c) cout << m[bottom][c] << "\n";
            bottom--;
        }
        if (left <= right) {
            for (int r = bottom; r >= top; --r) cout << m[r][left] << "\n";
            left++;
        }
    }
}

int main() {
    vector<vector<int>> m = {
        {1, 2, 3, 4, 5},
        {6, 7, 8, 9, 10},
        {11, 12, 13, 14, 15},
        {16, 17, 18, 19, 20}};
    spiral(m); // 1 2 3 4 5 10 15 20 19 18 17 16 11 6 7 8 9 14 13 12
    return 0;
}
