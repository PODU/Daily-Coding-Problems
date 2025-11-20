// Day 633: Sort a k-sorted (nearly sorted) array.
// Approach: min-heap of size k+1; pop smallest as we slide the window.
// Time: O(N log k), Space: O(k).
#include <bits/stdc++.h>
using namespace std;

vector<int> sortKSorted(vector<int> arr, int k) {
    priority_queue<int, vector<int>, greater<int>> pq;
    vector<int> res;
    res.reserve(arr.size());
    for (int i = 0; i < (int)arr.size(); i++) {
        pq.push(arr[i]);
        if ((int)pq.size() > k) {       // window holds at most k+1 elements
            res.push_back(pq.top());
            pq.pop();
        }
    }
    while (!pq.empty()) { res.push_back(pq.top()); pq.pop(); }
    return res;
}

int main() {
    vector<int> arr = {2, 1, 4, 3, 5}; // k = 1: each at most 1 from sorted spot
    auto sorted = sortKSorted(arr, 1);
    cout << "[";
    for (size_t i = 0; i < sorted.size(); i++) {
        cout << sorted[i];
        if (i + 1 < sorted.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
