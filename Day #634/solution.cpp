// Day 634: Largest rectangle in a histogram.
// Approach: monotonic increasing stack of bar indices; pop to settle areas.
// Time: O(N), Space: O(N).
#include <bits/stdc++.h>
using namespace std;

long long largestRectangle(const vector<int>& h) {
    int n = h.size();
    stack<int> st;
    long long best = 0;
    for (int i = 0; i <= n; i++) {
        int cur = (i == n) ? 0 : h[i];
        while (!st.empty() && h[st.top()] >= cur) {
            int height = h[st.top()]; st.pop();
            int left = st.empty() ? -1 : st.top();
            long long width = i - left - 1;
            best = max(best, (long long)height * width);
        }
        st.push(i);
    }
    return best;
}

int main() {
    vector<int> hist = {1, 3, 2, 5};
    cout << largestRectangle(hist) << "\n"; // 6
    return 0;
}
