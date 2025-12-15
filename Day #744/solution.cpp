// O(1) LFU cache. Each freq has a list<int> of keys (front = most recent);
// a map key->iterator gives O(1) removal. Eviction: least-frequent, then least-recent.
// Time: O(1) per get/set, Space: O(capacity).
#include <bits/stdc++.h>
using namespace std;

class LFUCache {
    int cap, minFreq = 0;
    unordered_map<int,int> val, freq;
    unordered_map<int, list<int>> buckets;                 // freq -> keys (front=recent)
    unordered_map<int, list<int>::iterator> pos;           // key -> iterator in bucket

    void bump(int key){
        int f = freq[key];
        buckets[f].erase(pos[key]);
        if(buckets[f].empty()){
            buckets.erase(f);
            if(minFreq == f) minFreq++;
        }
        freq[key] = f + 1;
        buckets[f+1].push_front(key);
        pos[key] = buckets[f+1].begin();
    }
public:
    LFUCache(int capacity): cap(capacity) {}

    // returns {found, value}
    pair<bool,int> get(int key){
        if(!val.count(key)) return {false, 0};
        bump(key);
        return {true, val[key]};
    }

    void set(int key, int value){
        if(cap <= 0) return;
        if(val.count(key)){ val[key] = value; bump(key); return; }
        if((int)val.size() >= cap){
            int evict = buckets[minFreq].back();
            buckets[minFreq].pop_back();
            val.erase(evict); freq.erase(evict); pos.erase(evict);
        }
        val[key] = value; freq[key] = 1;
        buckets[1].push_front(key);
        pos[key] = buckets[1].begin();
        minFreq = 1;
    }
};

void printGet(LFUCache& c, int key){
    auto r = c.get(key);
    if(r.first) cout << r.second << "\n"; else cout << "null\n";
}

int main(){
    LFUCache c(2);
    c.set(1,1); c.set(2,2);
    printGet(c,1);   // 1
    c.set(3,3);      // evicts key 2
    printGet(c,2);   // null
    printGet(c,3);   // 3
    c.set(4,4);      // evicts key 1
    printGet(c,1);   // null
    printGet(c,3);   // 3
    printGet(c,4);   // 4
    return 0;
}
