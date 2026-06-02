// Stack via a single MAX-heap keyed by a monotonic counter; newest key is largest so pops first (LIFO).
// push/pop O(log n), space O(n).
#include <bits/stdc++.h>
using namespace std;

class StackViaHeap {
    // max-heap of (counter, value)
    priority_queue<pair<long long,int>> pq;
    long long counter = 0;
public:
    void push(int x) { pq.push({counter++, x}); }
    int pop() {
        if (pq.empty()) throw runtime_error("pop from empty stack");
        int v = pq.top().second; pq.pop();
        return v;
    }
    bool empty() const { return pq.empty(); }
};

int main() {
    StackViaHeap s;
    s.push(1); s.push(2); s.push(3);
    vector<int> out;
    out.push_back(s.pop()); // 3
    out.push_back(s.pop()); // 2
    s.push(4);
    out.push_back(s.pop()); // 4
    out.push_back(s.pop()); // 1
    for (size_t i = 0; i < out.size(); ++i)
        cout << out[i] << (i + 1 < out.size() ? " " : "\n");
    return 0;
}
