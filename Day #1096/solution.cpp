// Day 1096: LFU Cache with O(1) get/set.
// Approach: hash maps key->(val,freq,iter) + freq->list(keys, LRU order) + minFreq.
// Time: O(1) per get/set. Space: O(n).
#include <bits/stdc++.h>
using namespace std;

class LFUCache {
    int cap, minFreq;
    unordered_map<int,int> vals;                 // key -> value
    unordered_map<int,int> freqs;                // key -> frequency
    unordered_map<int,list<int>> freqList;       // freq -> keys (front = most recent)
    unordered_map<int,list<int>::iterator> iters;// key -> position in freqList
public:
    LFUCache(int n): cap(n), minFreq(0) {}

    void touch(int key) {
        int f = freqs[key];
        freqList[f].erase(iters[key]);
        if (freqList[f].empty()) {
            freqList.erase(f);
            if (minFreq == f) minFreq++;
        }
        int nf = f + 1;
        freqs[key] = nf;
        freqList[nf].push_front(key);
        iters[key] = freqList[nf].begin();
    }

    // returns value or -1 if absent (use has() to disambiguate)
    int get(int key) {
        if (!vals.count(key)) return -1;
        touch(key);
        return vals[key];
    }
    bool has(int key){ return vals.count(key) > 0; }

    void set(int key, int value) {
        if (cap <= 0) return;
        if (vals.count(key)) { vals[key] = value; touch(key); return; }
        if ((int)vals.size() >= cap) {
            int evict = freqList[minFreq].back();
            freqList[minFreq].pop_back();
            if (freqList[minFreq].empty()) freqList.erase(minFreq);
            vals.erase(evict); freqs.erase(evict); iters.erase(evict);
        }
        vals[key] = value; freqs[key] = 1;
        freqList[1].push_front(key); iters[key] = freqList[1].begin();
        minFreq = 1;
    }
};

int main() {
    LFUCache c(2);
    c.set(1, 1);
    c.set(2, 2);
    cout << c.get(1) << "\n"; // 1
    c.set(3, 3);              // evicts key 2 (least frequently used)
    cout << (c.has(2) ? "2" : "null") << "\n"; // null
    cout << c.get(3) << "\n"; // 3
    return 0;
}
