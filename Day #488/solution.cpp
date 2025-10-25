// Day 488: Queue backed by a set of fixed-length arrays (blocks).
// Blocks of size B chained; head/tail indices roll over to next block.
// enqueue/dequeue amortized O(1), get_size O(1). Space O(n).
#include <bits/stdc++.h>
using namespace std;

class BlockQueue {
    static const int B = 4;          // fixed block length
    deque<array<int, B>> blocks;     // set of fixed-length arrays
    int head = 0, tail = 0, sz = 0;  // head index in front block, tail index in back block
public:
    void enqueue(int x) {
        if (blocks.empty() || tail == B) {     // allocate a new fixed block
            blocks.push_back(array<int, B>{});
            tail = 0;
        }
        blocks.back()[tail++] = x;
        sz++;
    }
    int dequeue() {
        if (sz == 0) throw runtime_error("empty");
        int x = blocks.front()[head++];
        sz--;
        if (head == B) {           // front block exhausted -> free it
            blocks.pop_front();
            head = 0;
        }
        if (blocks.size() == 1 && head == tail) { // single block fully consumed
            blocks.clear(); head = tail = 0;
        }
        return x;
    }
    int get_size() const { return sz; }
};

int main() {
    BlockQueue q;
    for (int i = 1; i <= 6; ++i) q.enqueue(i); // 1..6
    cout << "size=" << q.get_size() << "\n";   // 6
    cout << "deq=" << q.dequeue() << "\n";      // 1
    cout << "deq=" << q.dequeue() << "\n";      // 2
    q.enqueue(7); q.enqueue(8);
    cout << "size=" << q.get_size() << "\n";   // 6
    while (q.get_size() > 0) cout << q.dequeue() << " "; // 3 4 5 6 7 8
    cout << "\n";
    return 0;
}
