// Day 727: Running median over a stream.
// Approach: Max-heap for lower half, min-heap for upper half, kept balanced.
// Time: O(log n) per element, Space: O(n).
#include <bits/stdc++.h>
using namespace std;

void printMedian(double m) {
    if (m == floor(m)) cout << (long long)m << "\n";
    else cout << fixed << setprecision(1) << m << "\n";
}

int main() {
    vector<int> stream = {2, 1, 5, 7, 2, 0, 5};
    priority_queue<int> lo;                              // max-heap (lower half)
    priority_queue<int, vector<int>, greater<int>> hi;   // min-heap (upper half)
    for (int x : stream) {
        if (lo.empty() || x <= lo.top()) lo.push(x); else hi.push(x);
        if (lo.size() > hi.size() + 1) { hi.push(lo.top()); lo.pop(); }
        else if (hi.size() > lo.size()) { lo.push(hi.top()); hi.pop(); }
        if (lo.size() == hi.size()) printMedian((lo.top() + hi.top()) / 2.0);
        else printMedian(lo.top());
    }
    return 0;
}
