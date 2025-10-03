// All O(1) structure: doubly-linked list of count-buckets (increasing), each holds a key set; hashmap key->bucket.
// plus/minus move key to adjacent bucket. get_max=last bucket, get_min=first bucket. All O(1).
#include <bits/stdc++.h>
using namespace std;

struct Bucket {
    int count;
    list<string> keys;          // keys with this count (insertion order)
    unordered_map<string, list<string>::iterator> pos;
    Bucket(int c) : count(c) {}
};

class AllOne {
    list<Bucket> buckets;       // increasing count order
    unordered_map<string, list<Bucket>::iterator> keyBucket;

    void addKey(list<Bucket>::iterator b, const string& key) {
        b->keys.push_back(key);
        b->pos[key] = prev(b->keys.end());
        keyBucket[key] = b;
    }
    void removeKey(list<Bucket>::iterator b, const string& key) {
        b->keys.erase(b->pos[key]);
        b->pos.erase(key);
    }
public:
    void plus(const string& key) {
        if (!keyBucket.count(key)) {
            auto first = buckets.begin();
            if (first == buckets.end() || first->count != 1) {
                first = buckets.insert(buckets.begin(), Bucket(1));
            }
            addKey(first, key);
            return;
        }
        auto cur = keyBucket[key];
        auto nxt = next(cur);
        if (nxt == buckets.end() || nxt->count != cur->count + 1) {
            nxt = buckets.insert(nxt, Bucket(cur->count + 1));
        }
        addKey(nxt, key);
        removeKey(cur, key);
        if (cur->keys.empty()) buckets.erase(cur);
    }
    void minus(const string& key) {
        if (!keyBucket.count(key)) return;
        auto cur = keyBucket[key];
        if (cur->count == 1) {
            removeKey(cur, key);
            keyBucket.erase(key);
            if (cur->keys.empty()) buckets.erase(cur);
            return;
        }
        auto prv = (cur == buckets.begin()) ? buckets.end() : prev(cur);
        if (cur == buckets.begin() || prv->count != cur->count - 1) {
            prv = buckets.insert(cur, Bucket(cur->count - 1));
        }
        addKey(prv, key);
        removeKey(cur, key);
        if (cur->keys.empty()) buckets.erase(cur);
    }
    string get_max() { return buckets.empty() ? "" : *min_element(buckets.back().keys.begin(), buckets.back().keys.end()); }
    string get_min() { return buckets.empty() ? "" : *min_element(buckets.front().keys.begin(), buckets.front().keys.end()); }
};

int main() {
    AllOne a;
    a.plus("a"); a.plus("b"); a.plus("b");
    a.plus("c"); a.plus("c"); a.plus("c");
    cout << "max=" << a.get_max() << "\n";
    cout << "min=" << a.get_min() << "\n";
    a.minus("c"); a.minus("c");
    cout << "max=" << a.get_max() << "\n";
    return 0;
}
