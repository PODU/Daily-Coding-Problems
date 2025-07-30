// Day 53: FIFO queue from two stacks. Amortized O(1) per op.
// in-stack receives pushes; out-stack serves pops (refilled when empty).
#include <bits/stdc++.h>
using namespace std;

class Queue {
    stack<int> in, out;
public:
    void enqueue(int x) { in.push(x); }
    int dequeue() {
        if (out.empty())
            while (!in.empty()) { out.push(in.top()); in.pop(); }
        int v = out.top(); out.pop();
        return v;
    }
};

int main() {
    Queue q;
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    cout << q.dequeue() << endl; // 1
    cout << q.dequeue() << endl; // 2
    q.enqueue(4);
    cout << q.dequeue() << endl; // 3
    cout << q.dequeue() << endl; // 4
    return 0;
}
