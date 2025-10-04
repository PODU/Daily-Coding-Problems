// Day 368: KV store with update/get/max_key(val).
// kv maps key->value; byVal maps value->ordered set of keys, so max_key is the
// largest element of that set. update O(log n), get O(1) avg, max_key O(log n).
#include <bits/stdc++.h>
using namespace std;

class KVStore {
    unordered_map<int,int> kv;
    unordered_map<int,set<int>> byVal;
public:
    void update(int key, int val) {
        auto it = kv.find(key);
        if (it != kv.end()) {
            byVal[it->second].erase(key);
            if (byVal[it->second].empty()) byVal.erase(it->second);
        }
        kv[key] = val;
        byVal[val].insert(key);
    }
    optional<int> get(int key) {
        auto it = kv.find(key);
        if (it == kv.end()) return nullopt;
        return it->second;
    }
    optional<int> max_key(int val) {
        auto it = byVal.find(val);
        if (it == byVal.end() || it->second.empty()) return nullopt;
        return *it->second.rbegin();
    }
};

int main() {
    KVStore kv;
    kv.update(1, 1);
    kv.update(2, 1);
    cout << kv.max_key(1).value() << "\n"; // 2
    return 0;
}
