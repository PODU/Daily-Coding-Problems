// Day 1264: Largest rectangle of 1's in a binary matrix.
// Per-row histogram + largest-rectangle-in-histogram via monotonic stack. O(N*M) time, O(M) space.
#include <bits/stdc++.h>
using namespace std;

int largestInHistogram(const vector<int>& h) {
    int n = h.size(), best = 0;
    stack<int> st; // indices of increasing heights
    for (int i = 0; i <= n; ++i) {
        int cur = (i == n) ? 0 : h[i];
        while (!st.empty() && h[st.top()] >= cur) {
            int height = h[st.top()]; st.pop();
            int left = st.empty() ? -1 : st.top();
            best = max(best, height * (i - left - 1));
        }
        st.push(i);
    }
    return best;
}

int maximalRectangle(const vector<vector<int>>& m) {
    if (m.empty()) return 0;
    int cols = m[0].size(), best = 0;
    vector<int> h(cols, 0);
    for (const auto& row : m) {
        for (int j = 0; j < cols; ++j) h[j] = row[j] ? h[j] + 1 : 0;
        best = max(best, largestInHistogram(h));
    }
    return best;
}

int main() {
    vector<vector<int>> m = {{1,0,0,0},{1,0,1,1},{1,0,1,1},{0,1,0,0}};
    cout << maximalRectangle(m) << endl;
    return 0;
}
