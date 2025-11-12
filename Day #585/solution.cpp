// Build histogram heights per row; largest rectangle in histogram via monotonic stack.
// Time O(N*M), Space O(M).
#include <iostream>
#include <vector>
#include <stack>
using namespace std;

int largestInHistogram(const vector<int>& h) {
    stack<int> st;
    int best = 0, n = (int)h.size();
    for (int i = 0; i <= n; i++) {
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

int maximalRectangle(const vector<vector<int>>& mat) {
    if (mat.empty()) return 0;
    int m = (int)mat[0].size();
    vector<int> h(m, 0);
    int best = 0;
    for (const auto& row : mat) {
        for (int j = 0; j < m; j++) h[j] = row[j] ? h[j] + 1 : 0;
        best = max(best, largestInHistogram(h));
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
    cout << maximalRectangle(mat) << "\n";
    return 0;
}
