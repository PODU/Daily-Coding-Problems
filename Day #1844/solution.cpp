// Day 1844: Largest rectangle in a histogram via a monotonic increasing stack.
// Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

long long largestRectangle(vector<int> h) {
    h.push_back(0);  // sentinel to flush the stack
    stack<int> st;   // indices of increasing bar heights
    long long best = 0;
    for (int i = 0; i < (int)h.size(); i++) {
        while (!st.empty() && h[st.top()] >= h[i]) {
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
    cout << largestRectangle({1, 3, 2, 5}) << "\n";  // 6
}
