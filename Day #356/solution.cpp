// Queue built from a deque of fixed-capacity blocks (cap 4). Track head/tail block+offset and an O(1) size.
// enqueue/dequeue/get_size all amortized O(1) time; O(n) space.
#include <iostream>
#include <deque>
#include <vector>
using namespace std;

class BlockQueue {
    static const int CAP = 4;
    deque<vector<int>> blocks;
    int headOff = 0; // offset within front block
    int tailOff = 0; // next write offset within back block
    int sz = 0;
public:
    void enqueue(int v) {
        if (blocks.empty() || tailOff == CAP) {
            blocks.push_back(vector<int>(CAP));
            tailOff = 0;
        }
        blocks.back()[tailOff++] = v;
        sz++;
    }
    int dequeue() {
        int v = blocks.front()[headOff++];
        sz--;
        if (headOff == CAP) { // front block fully consumed
            blocks.pop_front();
            headOff = 0;
        }
        return v;
    }
    int get_size() const { return sz; }
};

int main() {
    BlockQueue q;
    for (int v : {1, 2, 3, 4, 5}) q.enqueue(v);
    cout << "size=" << q.get_size() << endl;
    cout << q.dequeue() << endl;
    cout << q.dequeue() << endl;
    cout << q.dequeue() << endl;
    cout << "size=" << q.get_size() << endl;
    return 0;
}
