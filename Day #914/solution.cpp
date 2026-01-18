// Queue via fixed-length array blocks (capacity 4). Deque of blocks; enqueue to tail block, dequeue from head block.
// Amortized O(1) per op; O(n) space.
#include <bits/stdc++.h>
using namespace std;

class BlockQueue {
    static const int CAP = 4;
    deque<vector<int>> blocks;
    int headIdx = 0; // index within head block
    int tailCount = 0; // filled slots in tail block
    int sz = 0;
public:
    void enqueue(int v) {
        if (blocks.empty() || tailCount == CAP) {
            blocks.push_back(vector<int>(CAP));
            tailCount = 0;
        }
        blocks.back()[tailCount++] = v;
        sz++;
    }
    int dequeue() {
        if (sz == 0) throw runtime_error("empty");
        int v = blocks.front()[headIdx++];
        sz--;
        if (headIdx == CAP || (blocks.size() == 1 && headIdx == tailCount)) {
            blocks.pop_front();
            headIdx = 0;
            if (blocks.empty()) tailCount = 0;
        }
        return v;
    }
    int get_size() const { return sz; }
};

int main() {
    BlockQueue q;
    for (int i = 1; i <= 10; i++) q.enqueue(i);
    cout << "Dequeued:";
    for (int i = 0; i < 3; i++) cout << " " << q.dequeue();
    cout << "\nSize: " << q.get_size() << "\n";
    return 0;
}
