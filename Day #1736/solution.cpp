// FIFO queue via two stacks (in/out); dequeue moves in->out when out empty. Amortized O(1) per op, O(n) space.
#include <bits/stdc++.h>
using namespace std;

class MyQueue {
    stack<int> in, out;
    void shift() { if (out.empty()) while (!in.empty()) { out.push(in.top()); in.pop(); } }
public:
    void enqueue(int x) { in.push(x); }
    int dequeue() { shift(); int v = out.top(); out.pop(); return v; }
};

int main() {
    MyQueue q;
    q.enqueue(1); q.enqueue(2); q.enqueue(3);
    cout << q.dequeue() << "\n";
    q.enqueue(4);
    cout << q.dequeue() << "\n";
    cout << q.dequeue() << "\n";
    cout << q.dequeue() << "\n";
    return 0;
}
