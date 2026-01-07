// Day 870: Largest rectangle of 1's in a binary matrix.
// Approach: build per-row histogram of consecutive 1's, apply largest-rectangle-in-histogram.
// Time: O(N*M), Space: O(M).
#include <bits/stdc++.h>
using namespace std;

int largestInHist(vector<int>& h) {
    stack<int> st;
    int best = 0, n = h.size();
    for (int i = 0; i <= n; i++) {
        int cur = (i == n) ? 0 : h[i];
        while (!st.empty() && h[st.top()] >= cur) {
            int height = h[st.top()]; st.pop();
            int width = st.empty() ? i : i - st.top() - 1;
            best = max(best, height * width);
        }
        st.push(i);
    }
    return best;
}

int maximalRectangle(vector<vector<int>>& mat) {
    if (mat.empty()) return 0;
    int m = mat[0].size();
    vector<int> h(m, 0);
    int best = 0;
    for (auto& row : mat) {
        for (int j = 0; j < m; j++) h[j] = row[j] ? h[j] + 1 : 0;
        best = max(best, largestInHist(h));
    }
    return best;
}

int main() {
    vector<vector<int>> mat = {
        {1,0,0,0},
        {1,0,1,1},
        {1,0,1,1},
        {0,1,0,0}
    };
    cout << maximalRectangle(mat) << endl; // 4
    return 0;
}
