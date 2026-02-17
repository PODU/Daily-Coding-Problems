// Sliding window maximum via monotonic decreasing deque of indices. Time O(n), Space O(k).
#include <bits/stdc++.h>
using namespace std;

vector<int> maxSlidingWindow(const vector<int>& a, int k) {
    deque<int> dq; // indices, values decreasing
    vector<int> res;
    for (int i = 0; i < (int)a.size(); i++) {
        if (!dq.empty() && dq.front() <= i - k) dq.pop_front();
        while (!dq.empty() && a[dq.back()] <= a[i]) dq.pop_back();
        dq.push_back(i);
        if (i >= k - 1) res.push_back(a[dq.front()]);
    }
    return res;
}

int main() {
    vector<int> a = {10, 5, 2, 7, 8, 7};
    vector<int> res = maxSlidingWindow(a, 3);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";
}
