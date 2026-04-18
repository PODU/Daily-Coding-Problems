// Spiral matrix traversal using four boundary pointers (top,bottom,left,right). O(N*M) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<vector<int>> mat = {
        {1,2,3,4,5},
        {6,7,8,9,10},
        {11,12,13,14,15},
        {16,17,18,19,20}
    };
    int top = 0, bottom = (int)mat.size() - 1;
    int left = 0, right = (int)mat[0].size() - 1;
    while (top <= bottom && left <= right) {
        for (int j = left; j <= right; ++j) cout << mat[top][j] << "\n";
        ++top;
        for (int i = top; i <= bottom; ++i) cout << mat[i][right] << "\n";
        --right;
        if (top <= bottom) {
            for (int j = right; j >= left; --j) cout << mat[bottom][j] << "\n";
            --bottom;
        }
        if (left <= right) {
            for (int i = bottom; i >= top; --i) cout << mat[i][left] << "\n";
            ++left;
        }
    }
    return 0;
}
