// Day 52: LRU cache with hashmap + doubly linked list. O(1) get/set.
// Time: O(1) per op, Space: O(n).
#include <bits/stdc++.h>
using namespace std;

class LRUCache {
    int cap;
    list<pair<int,int>> items;                       // front = MRU, back = LRU
    unordered_map<int, list<pair<int,int>>::iterator> pos;
public:
    LRUCache(int n): cap(n) {}

    int get(int key) {
        auto it = pos.find(key);
        if (it == pos.end()) return -1; // -1 denotes null
        items.splice(items.begin(), items, it->second);
        return it->second->second;
    }

    void set(int key, int value) {
        auto it = pos.find(key);
        if (it != pos.end()) {
            it->second->second = value;
            items.splice(items.begin(), items, it->second);
            return;
        }
        if ((int)items.size() == cap) {
            pos.erase(items.back().first);
            items.pop_back();
        }
        items.emplace_front(key, value);
        pos[key] = items.begin();
    }
};

static string show(int v) {
    return v == -1 ? "null" : to_string(v);
}

int main() {
    LRUCache c(2);
    c.set(1, 1);
    c.set(2, 2);
    cout << show(c.get(1)) << endl;   // 1
    c.set(3, 3);                      // evicts key 2
    cout << show(c.get(2)) << endl;   // null
    c.set(4, 4);                      // evicts key 1
    cout << show(c.get(1)) << endl;   // null
    cout << show(c.get(3)) << endl;   // 3
    cout << show(c.get(4)) << endl;   // 4
    return 0;
}
