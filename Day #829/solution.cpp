// Day 829: O(1) data structure with plus/minus/get_max/get_min.
// Doubly-linked list of count-buckets (each a set of keys) + key->bucket map.
// All operations O(1) time; O(K) space for K distinct keys.
#include <bits/stdc++.h>
using namespace std;

class AllOne {
    struct Bucket {
        long long count;
        unordered_set<string> keys;
        list<Bucket>::iterator self;  // unused placeholder
    };
    list<Bucket> buckets;  // front = lowest count, back = highest count
    unordered_map<string, list<Bucket>::iterator> keyBucket;

public:
    void plus(const string& key) {
        if (keyBucket.count(key)) {
            auto cur = keyBucket[key];
            long long newCount = cur->count + 1;
            auto nxt = next(cur);
            if (nxt == buckets.end() || nxt->count != newCount)
                nxt = buckets.insert(nxt, Bucket{newCount, {}, {}});
            nxt->keys.insert(key);
            keyBucket[key] = nxt;
            cur->keys.erase(key);
            if (cur->keys.empty()) buckets.erase(cur);
        } else {
            auto first = buckets.begin();
            if (first == buckets.end() || first->count != 1)
                first = buckets.insert(buckets.begin(), Bucket{1, {}, {}});
            first->keys.insert(key);
            keyBucket[key] = first;
        }
    }

    void minus(const string& key) {
        if (!keyBucket.count(key)) return;
        auto cur = keyBucket[key];
        if (cur->count == 1) {
            cur->keys.erase(key);
            keyBucket.erase(key);
            if (cur->keys.empty()) buckets.erase(cur);
            return;
        }
        long long newCount = cur->count - 1;
        list<Bucket>::iterator prv;
        if (cur == buckets.begin() || prev(cur)->count != newCount)
            prv = buckets.insert(cur, Bucket{newCount, {}, {}});
        else
            prv = prev(cur);
        prv->keys.insert(key);
        keyBucket[key] = prv;
        cur->keys.erase(key);
        if (cur->keys.empty()) buckets.erase(cur);
    }

    string get_max() {
        if (buckets.empty()) return "";
        return *min_element(buckets.back().keys.begin(), buckets.back().keys.end());
    }
    string get_min() {
        if (buckets.empty()) return "";
        return *min_element(buckets.front().keys.begin(), buckets.front().keys.end());
    }
};

int main() {
    AllOne ao;
    ao.plus("a");
    ao.plus("b");
    ao.plus("b");
    cout << "get_max: " << ao.get_max() << "\n";  // b
    cout << "get_min: " << ao.get_min() << "\n";  // a
    ao.minus("b");
    ao.minus("b");
    cout << "get_max: " << ao.get_max() << "\n";  // a
    return 0;
}
