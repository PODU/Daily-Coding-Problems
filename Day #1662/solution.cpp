// O(1) LFU cache: key->node map, freq->list(ordered by recency), minFreq pointer.
// get/set O(1); Space O(capacity). Evict least-freq, tie -> least-recently-used.
#include <bits/stdc++.h>
using namespace std;
struct LFU {
    int cap, minFreq = 0;
    unordered_map<int,int> vals, freqs;
    unordered_map<int,list<int>> lists;            // freq -> keys (front=MRU, back=LRU)
    unordered_map<int,list<int>::iterator> iters;  // key -> position in its freq list
    LFU(int c) : cap(c) {}
    void touch(int key) {
        int f = freqs[key];
        lists[f].erase(iters[key]);
        if (lists[f].empty()) { lists.erase(f); if (minFreq == f) minFreq++; }
        int nf = f + 1; freqs[key] = nf;
        lists[nf].push_front(key); iters[key] = lists[nf].begin();
    }
    int get(int key) {
        if (!vals.count(key)) return INT_MIN;
        touch(key);
        return vals[key];
    }
    void set(int key, int val) {
        if (cap <= 0) return;
        if (vals.count(key)) { vals[key] = val; touch(key); return; }
        if ((int)vals.size() >= cap) {
            int lru = lists[minFreq].back();
            lists[minFreq].pop_back();
            if (lists[minFreq].empty()) lists.erase(minFreq);
            vals.erase(lru); freqs.erase(lru); iters.erase(lru);
        }
        vals[key] = val; freqs[key] = 1;
        lists[1].push_front(key); iters[key] = lists[1].begin();
        minFreq = 1;
    }
};
void show(int v) { if (v == INT_MIN) cout << "null\n"; else cout << v << "\n"; }
int main() {
    LFU c(2);
    c.set(1,1); c.set(2,2);
    show(c.get(1));
    c.set(3,3);
    show(c.get(2));
    show(c.get(3));
    c.set(4,4);
    show(c.get(1));
    show(c.get(3));
    show(c.get(4));
    return 0;
}
