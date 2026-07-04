// Day 1760: Sliding window maximum.
// Approach: monotonic deque of indices (decreasing values). O(n) time, O(k) space.
#include <iostream>
#include <vector>
#include <deque>
using namespace std;

vector<int> maxSlidingWindow(const vector<int>& a, int k) {
    vector<int> res;
    deque<int> dq; // indices, values decreasing
    for (int i = 0; i < (int)a.size(); ++i) {
        if (!dq.empty() && dq.front() <= i - k) dq.pop_front();
        while (!dq.empty() && a[dq.back()] <= a[i]) dq.pop_back();
        dq.push_back(i);
        if (i >= k - 1) res.push_back(a[dq.front()]);
    }
    return res;
}

int main() {
    vector<int> a = {10, 5, 2, 7, 8, 7};
    int k = 3;
    vector<int> res = maxSlidingWindow(a, k);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i)
        cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
