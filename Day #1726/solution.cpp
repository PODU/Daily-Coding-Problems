// Sort a k-sorted array (each element <= k from its sorted position).
// Min-heap of size k+1: pop the min as the next sorted element. O(N log k) time, O(k) space.
#include <bits/stdc++.h>
using namespace std;

vector<int> sortKSorted(const vector<int>& arr, int k) {
    priority_queue<int, vector<int>, greater<int>> minHeap;
    vector<int> result;
    result.reserve(arr.size());
    int n = (int)arr.size();
    int i = 0;
    for (; i <= k && i < n; i++) minHeap.push(arr[i]);
    for (; i < n; i++) {
        result.push_back(minHeap.top()); minHeap.pop();
        minHeap.push(arr[i]);
    }
    while (!minHeap.empty()) { result.push_back(minHeap.top()); minHeap.pop(); }
    return result;
}

int main() {
    vector<int> arr = {2, 1, 4, 3, 6, 5}; // k-sorted with k = 2
    vector<int> sorted = sortKSorted(arr, 2);
    for (size_t j = 0; j < sorted.size(); j++) {
        cout << sorted[j];
        if (j + 1 < sorted.size()) cout << " ";
    }
    cout << "\n";
    return 0;
}
