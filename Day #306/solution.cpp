// Day 306: Sort a k-sorted array with a min-heap of size k+1. O(N log k).
#include <bits/stdc++.h>
using namespace std;
vector<int> sortK(vector<int> arr, int k) {
    priority_queue<int, vector<int>, greater<int>> pq;
    vector<int> res; size_t i = 0;
    for (; i < arr.size() && i <= (size_t)k; i++) pq.push(arr[i]);
    for (; i < arr.size(); i++) { res.push_back(pq.top()); pq.pop(); pq.push(arr[i]); }
    while (!pq.empty()) { res.push_back(pq.top()); pq.pop(); }
    return res;
}
int main() {
    vector<int> arr = {6, 5, 3, 2, 8, 10, 9}; int k = 3;
    auto r = sortK(arr, k);
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? " " : "");
    cout << "\n"; // 2 3 5 6 8 9 10
}
