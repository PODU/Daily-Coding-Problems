// Day 858: Running median of a stream.
// Approach: max-heap (lower half) + min-heap (upper half), balanced sizes.
// Time: O(n log n) total, Space: O(n).
#include <bits/stdc++.h>
using namespace std;

void printMedian(double m) {
    if (m == floor(m)) cout << (long long)m << "\n";
    else cout << m << "\n";
}

int main() {
    vector<int> stream = {2, 1, 5, 7, 2, 0, 5};
    priority_queue<int> lo;                              // max-heap
    priority_queue<int, vector<int>, greater<int>> hi;   // min-heap

    for (int x : stream) {
        if (lo.empty() || x <= lo.top()) lo.push(x);
        else hi.push(x);
        // rebalance
        if (lo.size() > hi.size() + 1) { hi.push(lo.top()); lo.pop(); }
        else if (hi.size() > lo.size()) { lo.push(hi.top()); hi.pop(); }

        double med = lo.size() > hi.size() ? lo.top() : (lo.top() + hi.top()) / 2.0;
        printMedian(med);
    }
    return 0;
}
