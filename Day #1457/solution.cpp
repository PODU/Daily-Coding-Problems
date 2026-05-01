// LFU cache: unordered_map key->iterator/node, map freq->list (DLL), track minFreq. O(1) per op.
// Time: O(1) get/set. Space: O(capacity).
#include <bits/stdc++.h>
using namespace std;

class LFUCache {
    struct Node { int key, val, freq; };
    int cap;
    int minFreq;
    // freq -> list of nodes (front = most recently used)
    unordered_map<int, list<Node>> freqList;
    // key -> iterator into its freq list
    unordered_map<int, list<Node>::iterator> pos;

    void touch(list<Node>::iterator it) {
        int f = it->freq;
        Node node = *it;
        freqList[f].erase(it);
        if (freqList[f].empty()) {
            freqList.erase(f);
            if (minFreq == f) minFreq++;
        }
        node.freq = f + 1;
        freqList[f + 1].push_front(node);
        pos[node.key] = freqList[f + 1].begin();
    }

public:
    LFUCache(int capacity) : cap(capacity), minFreq(0) {}

    // returns true if found, sets out
    bool get(int key, int &out) {
        auto pit = pos.find(key);
        if (pit == pos.end()) return false;
        out = pit->second->val;
        touch(pit->second);
        return true;
    }

    void set(int key, int value) {
        if (cap <= 0) return;
        auto pit = pos.find(key);
        if (pit != pos.end()) {
            pit->second->val = value;
            touch(pit->second);
            return;
        }
        if ((int)pos.size() >= cap) {
            auto &lst = freqList[minFreq];
            Node victim = lst.back();
            lst.pop_back();
            if (lst.empty()) freqList.erase(minFreq);
            pos.erase(victim.key);
        }
        freqList[1].push_front(Node{key, value, 1});
        pos[key] = freqList[1].begin();
        minFreq = 1;
    }
};

int main() {
    LFUCache cache(2);
    cache.set(1, 1);
    cache.set(2, 2);
    int out;
    cout << (cache.get(1, out) ? to_string(out) : "null") << "\n"; // 1
    cache.set(3, 3); // evicts key 2
    cout << (cache.get(2, out) ? to_string(out) : "null") << "\n"; // null
    cache.get(3, out); // access key 3 (raises its freq) so key 1 becomes LFU/LRU victim
    cache.set(4, 4); // evicts key 1
    cout << (cache.get(1, out) ? to_string(out) : "null") << "\n"; // null
    cout << (cache.get(3, out) ? to_string(out) : "null") << "\n"; // 3
    cout << (cache.get(4, out) ? to_string(out) : "null") << "\n"; // 4
    return 0;
}
