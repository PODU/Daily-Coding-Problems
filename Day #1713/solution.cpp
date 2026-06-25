// Unrolled/paged queue: list of fixed-length blocks; head/tail indices. Amortized O(1) per op.
#include <bits/stdc++.h>
using namespace std;

class BlockQueue {
    static const int BS = 4;
    deque<vector<int>> blocks; // each block has fixed capacity BS
    int head = 0; // index within front block
    int tail = 0; // next free index within back block
    int sz = 0;
public:
    void enqueue(int x) {
        if (blocks.empty() || tail == BS) {
            blocks.push_back(vector<int>(BS));
            tail = 0;
        }
        blocks.back()[tail++] = x;
        sz++;
    }
    int dequeue() {
        if (sz == 0) throw runtime_error("empty");
        int x = blocks.front()[head++];
        sz--;
        if (head == BS || (blocks.size() == 1 && head == tail)) {
            blocks.pop_front();
            head = 0;
            if (blocks.empty()) tail = 0;
        }
        return x;
    }
    int get_size() const { return sz; }
    int num_blocks() const { return (int)blocks.size(); }
};

int main() {
    BlockQueue q;
    for (int i = 1; i <= 10; i++) q.enqueue(i);
    cout << "size after enqueue 1..10: " << q.get_size() << "\n";
    cout << "blocks allocated: " << q.num_blocks() << "\n";
    int d1 = q.dequeue(), d2 = q.dequeue(), d3 = q.dequeue();
    cout << "dequeue 3: " << d1 << " " << d2 << " " << d3 << "\n";
    cout << "size: " << q.get_size() << "\n";
    q.enqueue(11);
    cout << "enqueue 11, size: " << q.get_size() << "\n";
    cout << "dequeue rest:";
    while (q.get_size() > 0) cout << " " << q.dequeue();
    cout << "\n";
    cout << "size: " << q.get_size() << "\n";
    return 0;
}
