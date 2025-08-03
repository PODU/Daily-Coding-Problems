// Spiral order print via boundary shrinking (top/bottom/left/right). Time O(N*M), Space O(1) extra.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<vector<int>> m = {
        {1,2,3,4,5},
        {6,7,8,9,10},
        {11,12,13,14,15},
        {16,17,18,19,20}
    };
    int n = m.size(), cols = m[0].size();
    int top = 0, bottom = n - 1, left = 0, right = cols - 1;
    while (top <= bottom && left <= right) {
        for (int j = left; j <= right; ++j) cout << m[top][j] << "\n";
        ++top;
        for (int i = top; i <= bottom; ++i) cout << m[i][right] << "\n";
        --right;
        if (top <= bottom) {
            for (int j = right; j >= left; --j) cout << m[bottom][j] << "\n";
            --bottom;
        }
        if (left <= right) {
            for (int i = bottom; i >= top; --i) cout << m[i][left] << "\n";
            ++left;
        }
    }
    return 0;
}
