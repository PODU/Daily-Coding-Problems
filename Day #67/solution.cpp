// LFU cache: key->(value,freq), freq->ordered list of keys (LRU within freq), track minFreq. O(1) get/set.
#include <bits/stdc++.h>
using namespace std;

class LFUCache {
    int cap, minFreq;
    unordered_map<int,pair<int,int>> kv;            // key -> {value, freq}
    unordered_map<int,list<int>> freqList;          // freq -> keys (front = most recent)
    unordered_map<int,list<int>::iterator> pos;     // key -> iterator in freqList[freq]

    void touch(int key) {
        int f = kv[key].second;
        freqList[f].erase(pos[key]);
        if (freqList[f].empty()) {
            freqList.erase(f);
            if (minFreq == f) minFreq = f + 1;
        }
        int nf = f + 1;
        kv[key].second = nf;
        freqList[nf].push_front(key);
        pos[key] = freqList[nf].begin();
    }
public:
    LFUCache(int capacity) : cap(capacity), minFreq(0) {}

    int get(int key) {
        auto it = kv.find(key);
        if (it == kv.end()) return -1;
        touch(key);
        return it->second.first;
    }

    void set(int key, int value) {
        if (cap <= 0) return;
        auto it = kv.find(key);
        if (it != kv.end()) {
            it->second.first = value;
            touch(key);
            return;
        }
        if ((int)kv.size() >= cap) {
            int evict = freqList[minFreq].back();      // least recently used at min freq
            freqList[minFreq].pop_back();
            if (freqList[minFreq].empty()) freqList.erase(minFreq);
            pos.erase(evict);
            kv.erase(evict);
        }
        kv[key] = {value, 1};
        freqList[1].push_front(key);
        pos[key] = freqList[1].begin();
        minFreq = 1;
    }
};

int main() {
    LFUCache c(2);
    c.set(1,1);
    c.set(2,2);
    cout << c.get(1) << "\n";                 // 1
    c.set(3,3);                               // evicts key 2
    int v2 = c.get(2);
    cout << (v2 == -1 ? "null" : to_string(v2)) << "\n"; // null
    cout << c.get(3) << "\n";                 // 3
    c.set(4,4);                               // evicts key 1
    int v1 = c.get(1);
    cout << (v1 == -1 ? "null" : to_string(v1)) << "\n"; // null
    cout << c.get(3) << "\n";                 // 3
    cout << c.get(4) << "\n";                 // 4
    return 0;
}
