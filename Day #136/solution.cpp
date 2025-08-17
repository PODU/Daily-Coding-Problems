// Largest rectangle of 1's: build per-row histogram, largest-rectangle-in-histogram via monotonic stack.
// Time O(N*M), Space O(M).
#include <bits/stdc++.h>
using namespace std;

int maximalRectangle(vector<vector<int>>& mat) {
    if (mat.empty() || mat[0].empty()) return 0;
    int n = mat.size(), m = mat[0].size(), best = 0;
    vector<int> h(m, 0);
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) h[j] = mat[i][j] ? h[j] + 1 : 0;
        stack<int> st;
        for (int j = 0; j <= m; j++) {
            int cur = (j == m) ? 0 : h[j];
            while (!st.empty() && h[st.top()] >= cur) {
                int height = h[st.top()]; st.pop();
                int width = st.empty() ? j : j - st.top() - 1;
                best = max(best, height * width);
            }
            st.push(j);
        }
    }
    return best;
}

int main() {
    vector<vector<int>> mat = {{1,0,0,0},{1,0,1,1},{1,0,1,1},{0,1,0,0}};
    cout << maximalRectangle(mat) << endl;
    return 0;
}
