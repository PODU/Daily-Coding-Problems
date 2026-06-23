// Largest rectangle of 1s: per-row histogram + largest-rectangle-in-histogram via stack.
// Time O(N*M), Space O(M).
#include <bits/stdc++.h>
using namespace std;

int largestInHist(vector<int>& h) {
    int n = h.size(), best = 0;
    stack<int> st;
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

int main() {
    vector<vector<int>> mat = {{1,0,0,0},{1,0,1,1},{1,0,1,1},{0,1,0,0}};
    int n = mat.size(), m = mat[0].size(), best = 0;
    vector<int> h(m, 0);
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) h[j] = mat[i][j] ? h[j] + 1 : 0;
        best = max(best, largestInHist(h));
    }
    cout << best << "\n";
    return 0;
}
