// Running median with two heaps (max-heap low, min-heap high). O(log n) per insert.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> nums = {2, 1, 5, 7, 2, 0, 5};
    priority_queue<int> low;                          // max-heap
    priority_queue<int, vector<int>, greater<int>> high; // min-heap

    for (int x : nums) {
        if (low.empty() || x <= low.top()) low.push(x);
        else high.push(x);
        // rebalance
        if (low.size() > high.size() + 1) {
            high.push(low.top()); low.pop();
        } else if (high.size() > low.size()) {
            low.push(high.top()); high.pop();
        }

        if (low.size() == high.size()) {
            long sum = (long)low.top() + high.top();
            if (sum % 2 == 0) cout << sum / 2 << "\n";
            else cout << (sum / 2.0) << "\n";
        } else {
            cout << low.top() << "\n";
        }
    }
    return 0;
}
