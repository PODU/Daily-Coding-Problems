// Stack via max-heap: tag each push with increasing priority; heap pops highest priority (most recent). O(log n)/op, O(n) space.
#include <bits/stdc++.h>
using namespace std;

class HeapStack {
    priority_queue<pair<long long,int>> pq; // (priority, value)
    long long counter = 0;
public:
    void push(int v) { pq.push({counter++, v}); }
    int pop() {
        if (pq.empty()) throw runtime_error("pop from empty stack");
        int v = pq.top().second; pq.pop(); return v;
    }
};

int main() {
    HeapStack s;
    s.push(1); s.push(2); s.push(3);
    cout << s.pop() << "\n"; // 3
    cout << s.pop() << "\n"; // 2
    s.push(4);
    cout << s.pop() << "\n"; // 4
    cout << s.pop() << "\n"; // 1
    return 0;
}
