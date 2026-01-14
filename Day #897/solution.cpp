// Queue via two stacks: enqueue->inStack; dequeue moves all to outStack when empty.
// Amortized O(1) per op, Space O(n).
#include <bits/stdc++.h>
using namespace std;

class Queue {
    stack<int> inStack, outStack;
public:
    void enqueue(int x) { inStack.push(x); }
    int dequeue() {
        if (outStack.empty()) {
            while (!inStack.empty()) {
                outStack.push(inStack.top());
                inStack.pop();
            }
        }
        int v = outStack.top();
        outStack.pop();
        return v;
    }
};

int main() {
    Queue q;
    q.enqueue(1);
    q.enqueue(2);
    cout << q.dequeue() << "\n";
    q.enqueue(3);
    cout << q.dequeue() << "\n";
    cout << q.dequeue() << "\n";
    return 0;
}
