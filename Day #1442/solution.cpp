// Day 1442: Implement a stack using only a max-heap.
// Approach: tag each item with an increasing counter as its key; the heap's
// max key is always the most recently pushed item. push/pop O(log n).
#include <bits/stdc++.h>
using namespace std;

class Stack {
    // max-heap on (counter, value)
    priority_queue<pair<long long, int>> heap;
    long long counter = 0;
public:
    void push(int item) { heap.push({counter++, item}); }
    int pop() {
        if (heap.empty()) throw runtime_error("pop from empty stack");
        int v = heap.top().second;
        heap.pop();
        return v;
    }
    bool empty() const { return heap.empty(); }
};

int main() {
    Stack s;
    s.push(1); s.push(2); s.push(3);
    int a = s.pop(), b = s.pop(), c = s.pop(); // sequence pops explicitly
    cout << a << " " << b << " " << c << "\n";  // 3 2 1
    return 0;
}
