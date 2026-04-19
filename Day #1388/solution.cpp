// FIFO queue via two stacks (in/out); amortized O(1) per op, O(n) space.
#include <iostream>
#include <stack>
using namespace std;

class MyQueue {
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
    MyQueue q;
    q.enqueue(1); q.enqueue(2); q.enqueue(3);
    cout << q.dequeue() << "\n";          // 1
    q.enqueue(4);
    cout << q.dequeue() << "\n";          // 2
    cout << q.dequeue() << "\n";          // 3
    cout << q.dequeue() << "\n";          // 4
    return 0;
}
