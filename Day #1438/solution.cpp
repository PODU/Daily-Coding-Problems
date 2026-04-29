// Day 1438: Largest rectangle in a histogram.
// Approach: monotonic increasing stack of bar indices; pop to compute areas.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

long long largestRectangle(const vector<int>& heights) {
    stack<int> st; // indices with increasing heights
    long long best = 0;
    int n = heights.size();
    for (int i = 0; i <= n; ++i) {
        int h = (i == n) ? 0 : heights[i];
        while (!st.empty() && heights[st.top()] >= h) {
            int top = st.top(); st.pop();
            int width = st.empty() ? i : i - st.top() - 1;
            best = max(best, (long long)heights[top] * width);
        }
        st.push(i);
    }
    return best;
}

int main() {
    vector<int> h = {1, 3, 2, 5};
    cout << largestRectangle(h) << endl; // 6
    return 0;
}
