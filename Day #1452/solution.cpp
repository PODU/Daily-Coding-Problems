// Day 1452: Sort a k-sorted array (each element <= k from its place) using a
// min-heap of size k+1. Time O(N log k), Space O(k).
#include <bits/stdc++.h>
using namespace std;

vector<int> sortKSorted(vector<int> a, int k) {
    priority_queue<int, vector<int>, greater<int>> heap; // min-heap
    vector<int> out;
    out.reserve(a.size());
    int n = a.size();
    for (int i = 0; i < n; i++) {
        heap.push(a[i]);
        if ((int)heap.size() > k) { out.push_back(heap.top()); heap.pop(); }
    }
    while (!heap.empty()) { out.push_back(heap.top()); heap.pop(); }
    return out;
}

int main() {
    vector<int> a = {2, 6, 3, 12, 56, 8};
    int k = 3;
    vector<int> s = sortKSorted(a, k);
    cout << "[";
    for (size_t i = 0; i < s.size(); i++) cout << s[i] << (i + 1 < s.size() ? ", " : "");
    cout << "]\n"; // [2, 3, 6, 8, 12, 56]
    return 0;
}
