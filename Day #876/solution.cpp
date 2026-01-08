// Sort a k-sorted array with a min-heap of size k+1. Time O(N log k), Space O(k).
#include <bits/stdc++.h>
using namespace std;

vector<int> sortKSorted(vector<int> a, int k){
    priority_queue<int, vector<int>, greater<int>> heap;
    vector<int> res;
    res.reserve(a.size());
    int n = a.size();
    for(int i = 0; i < n; i++){
        heap.push(a[i]);
        if((int)heap.size() > k){
            res.push_back(heap.top());
            heap.pop();
        }
    }
    while(!heap.empty()){ res.push_back(heap.top()); heap.pop(); }
    return res;
}

int main(){
    vector<int> a = {6, 5, 3, 2, 8, 10, 9};
    int k = 3;
    vector<int> sorted = sortKSorted(a, k);
    cout << "[";
    for(size_t i = 0; i < sorted.size(); i++) cout << sorted[i] << (i + 1 < sorted.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
