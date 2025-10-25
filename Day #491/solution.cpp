// Day 491: Running median of a stream.
// Two heaps: a max-heap keeps the lower half, a min-heap the upper half; rebalance so
// the lower half has equal size or one extra, so the median is its top.
// Time: O(log n) per element, Space: O(n).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> stream = {2, 1, 5, 7, 2, 0, 5};
    priority_queue<int> lo;                             // max-heap: lower half
    priority_queue<int, vector<int>, greater<int>> hi;  // min-heap: upper half
    for (int x : stream) {
        if (lo.empty() || x <= lo.top()) lo.push(x); else hi.push(x);
        if (lo.size() > hi.size() + 1) { hi.push(lo.top()); lo.pop(); }
        else if (hi.size() > lo.size()) { lo.push(hi.top()); hi.pop(); }
        double m = (lo.size() == hi.size()) ? (lo.top() + hi.top()) / 2.0 : lo.top();
        // 2, 1.5, 2, 3.5, 2, 2, 2
        if (m == floor(m)) cout << (long long)m << "\n";
        else cout << m << "\n";
    }
    return 0;
}
