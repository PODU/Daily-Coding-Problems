// Day 734: Time-travel map; get(key,t) returns value set at the most recent time <= t.
// Approach: per key an ordered map time->value; get uses upper_bound then steps back.
// Time: set O(log n), get O(log n). Space: O(n).
#include <bits/stdc++.h>
using namespace std;

class TimeMap {
    unordered_map<int, map<int,int>> store;
public:
    void set(int key, int value, int time) { store[key][time] = value; }
    // returns {found, value}
    pair<bool,int> get(int key, int time) {
        auto it = store.find(key);
        if (it == store.end()) return {false, 0};
        auto& m = it->second;
        auto u = m.upper_bound(time);
        if (u == m.begin()) return {false, 0};
        return {true, prev(u)->second};
    }
};

void show(pair<bool,int> r) {
    if (r.first) cout << r.second << "\n"; else cout << "null\n";
}

int main() {
    TimeMap d1; d1.set(1,1,0); d1.set(1,2,2);
    show(d1.get(1,1)); // 1
    show(d1.get(1,3)); // 2
    TimeMap d2; d2.set(1,1,5);
    show(d2.get(1,0)); // null
    show(d2.get(1,10)); // 1
    TimeMap d3; d3.set(1,1,0); d3.set(1,2,0);
    show(d3.get(1,0)); // 2
    return 0;
}
