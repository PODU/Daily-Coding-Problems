// Day 438: Stack via a max-heap. Each push tags the item with an increasing
// counter; the heap's max counter is the most-recent item. push/pop O(log n).
#include <bits/stdc++.h>
using namespace std;

class StackFromHeap {
    priority_queue<pair<long long,int>> heap; // (counter, value)
    long long counter = 0;
public:
    void push(int item) { heap.push({counter++, item}); }
    int pop() {
        if (heap.empty()) throw runtime_error("stack is empty");
        int v = heap.top().second; heap.pop(); return v;
    }
    bool empty() const { return heap.empty(); }
};

int main() {
    StackFromHeap s;
    s.push(1); s.push(2); s.push(3);
    cout << s.pop() << "\n"; // 3
    cout << s.pop() << "\n"; // 2
    s.push(4);
    cout << s.pop() << "\n"; // 4
    cout << s.pop() << "\n"; // 1
    return 0;
}
