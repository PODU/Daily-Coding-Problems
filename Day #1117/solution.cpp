// Day 1117 - Three stacks backed by a single list
// Each entry stores (value, prev_index); per-stack heads + free list give O(1)
// push/pop sharing one list. Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Stack3 {
    vector<pair<int,int>> list; // (value, prev)
    int heads[3] = {-1, -1, -1};
    vector<int> freeList;

    void push(int item, int s) {
        int idx;
        if (!freeList.empty()) {
            idx = freeList.back(); freeList.pop_back();
            list[idx] = {item, heads[s]};
        } else {
            idx = list.size();
            list.push_back({item, heads[s]});
        }
        heads[s] = idx;
    }

    int pop(int s) {
        int idx = heads[s];
        if (idx == -1) throw runtime_error("pop from empty stack");
        auto [value, prev] = list[idx];
        heads[s] = prev;
        freeList.push_back(idx);
        return value;
    }
};

int main() {
    Stack3 s;
    s.push(1, 0); s.push(2, 0);
    s.push(3, 1);
    s.push(4, 2); s.push(5, 2);
    cout << "pop(0): " << s.pop(0) << "\n"; // 2
    cout << "pop(0): " << s.pop(0) << "\n"; // 1
    cout << "pop(2): " << s.pop(2) << "\n"; // 5
    cout << "pop(1): " << s.pop(1) << "\n"; // 3
    cout << "pop(2): " << s.pop(2) << "\n"; // 4
    return 0;
}
