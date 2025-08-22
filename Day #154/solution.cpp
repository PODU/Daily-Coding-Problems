// Day 154: Stack using only a max-heap. Tag each item with an increasing
// priority so the heap always pops the most recent. push/pop O(log n).
#include <bits/stdc++.h>
using namespace std;

class HeapStack {
    priority_queue<pair<long long,int>> heap; // (priority, value)
    long long counter = 0;
public:
    void push(int item) { heap.push({counter++, item}); }
    int pop() {
        if (heap.empty()) throw runtime_error("pop from empty stack");
        int v = heap.top().second; heap.pop();
        return v;
    }
    bool empty() const { return heap.empty(); }
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
