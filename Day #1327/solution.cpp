// Day 1327: Queue backed by a deque of fixed-length blocks (chunks).
// enqueue appends to the tail block (new block when full); dequeue pops from the head block. Amortized O(1).
#include <bits/stdc++.h>
using namespace std;

class BlockQueue {
    static const int BLOCK = 4;
    deque<vector<int>> blocks;
    int head = 0;   // read index inside front block
    int sz = 0;
public:
    void enqueue(int x) {
        if (blocks.empty() || blocks.back().size() == BLOCK) blocks.push_back({});
        blocks.back().push_back(x);
        sz++;
    }
    int dequeue() {
        if (sz == 0) throw runtime_error("empty");
        int x = blocks.front()[head++];
        if (head == (int)blocks.front().size()) { blocks.pop_front(); head = 0; }
        sz--;
        return x;
    }
    int get_size() const { return sz; }
};

int main() {
    BlockQueue q;
    for (int i = 1; i <= 5; i++) q.enqueue(i);
    cout << q.dequeue() << endl;  // 1
    cout << q.dequeue() << endl;  // 2
    cout << q.get_size() << endl; // 3
    return 0;
}
