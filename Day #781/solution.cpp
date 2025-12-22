// Largest rectangle in a histogram.
// Monotonic increasing stack of bar indices; pop when a lower bar arrives. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

long long largestRectangle(const vector<int>& h) {
    int n = h.size();
    stack<int> st; // indices of increasing heights
    long long best = 0;
    for (int i = 0; i <= n; ++i) {
        int cur = (i == n) ? 0 : h[i];
        while (!st.empty() && h[st.top()] >= cur) {
            int height = h[st.top()];
            st.pop();
            int left = st.empty() ? -1 : st.top();
            long long width = i - left - 1;
            best = max(best, (long long)height * width);
        }
        st.push(i);
    }
    return best;
}

int main() {
    vector<int> heights = {1, 3, 2, 5};
    cout << largestRectangle(heights) << "\n";
    return 0;
}
