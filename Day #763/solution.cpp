// Day 763: Sliding window maximum via a monotonic decreasing deque of indices.
// Time: O(n), Space: O(k). Prints each window max as it is computed.
#include <bits/stdc++.h>
using namespace std;

void slidingMax(vector<int>& a, int k) {
    deque<int> dq;                 // indices, values decreasing
    vector<int> out;
    for (int i = 0; i < (int)a.size(); ++i) {
        while (!dq.empty() && a[dq.back()] <= a[i]) dq.pop_back();
        dq.push_back(i);
        if (dq.front() <= i - k) dq.pop_front();   // drop out-of-window
        if (i >= k - 1) out.push_back(a[dq.front()]);
    }
    cout << "[";
    for (size_t i = 0; i < out.size(); ++i)
        cout << out[i] << (i + 1 < out.size() ? ", " : "");
    cout << "]\n";
}

int main() {
    vector<int> a = {10, 5, 2, 7, 8, 7};
    slidingMax(a, 3);   // [10, 7, 8, 8]
    return 0;
}
