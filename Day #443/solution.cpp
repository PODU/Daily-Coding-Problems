// Day 443: FIFO queue from two stacks. Amortized O(1) per op: push onto `in`,
// pop from `out`, refilling `out` from `in` when empty.
#include <bits/stdc++.h>
using namespace std;

class QueueTwoStacks {
    stack<int> in, out;
public:
    void enqueue(int x) { in.push(x); }
    int dequeue() {
        if (out.empty()) {
            if (in.empty()) throw runtime_error("queue is empty");
            while (!in.empty()) { out.push(in.top()); in.pop(); }
        }
        int v = out.top(); out.pop(); return v;
    }
    bool empty() const { return in.empty() && out.empty(); }
};

int main() {
    QueueTwoStacks q;
    q.enqueue(1); q.enqueue(2); q.enqueue(3);
    cout << q.dequeue() << "\n"; // 1
    cout << q.dequeue() << "\n"; // 2
    q.enqueue(4);
    cout << q.dequeue() << "\n"; // 3
    cout << q.dequeue() << "\n"; // 4
    return 0;
}
