// LRU cache via hash map + intrusive doubly linked list (std::list). O(1) per get/set. Space O(capacity).
#include <bits/stdc++.h>
using namespace std;

class LRUCache {
    int cap;
    list<pair<int,int>> items; // front = most recently used
    unordered_map<int, list<pair<int,int>>::iterator> pos;
public:
    explicit LRUCache(int n) : cap(n) {}

    // returns true if found, value via out param
    bool get(int key, int &out) {
        auto it = pos.find(key);
        if (it == pos.end()) return false;
        items.splice(items.begin(), items, it->second);
        out = it->second->second;
        return true;
    }

    void set(int key, int value) {
        auto it = pos.find(key);
        if (it != pos.end()) {
            it->second->second = value;
            items.splice(items.begin(), items, it->second);
            return;
        }
        if ((int)items.size() == cap) {
            auto last = items.back();
            pos.erase(last.first);
            items.pop_back();
        }
        items.emplace_front(key, value);
        pos[key] = items.begin();
    }
};

void printGet(LRUCache &c, int key) {
    int v;
    if (c.get(key, v)) cout << v << "\n";
    else cout << "null\n";
}

int main() {
    LRUCache c(2);
    c.set(1, 1);
    c.set(2, 2);
    printGet(c, 1);   // 1
    c.set(3, 3);      // evicts 2
    printGet(c, 2);   // null
    c.set(4, 4);      // evicts 1
    printGet(c, 1);   // null
    printGet(c, 3);   // 3
    printGet(c, 4);   // 4
    return 0;
}
