// Largest rectangle in histogram via monotonic increasing stack. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

long long largestRectangle(const vector<int>& heights) {
    stack<int> st; // indices of increasing bars
    long long best = 0;
    int n = (int)heights.size();
    for (int i = 0; i <= n; i++) {
        int h = (i < n) ? heights[i] : 0;
        while (!st.empty() && heights[st.top()] >= h) {
            int height = heights[st.top()]; st.pop();
            int width = st.empty() ? i : i - st.top() - 1;
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
