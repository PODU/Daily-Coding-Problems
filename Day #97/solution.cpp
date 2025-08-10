// Day 97: Time-versioned map. Each key keeps an ordered map time->value; get does
// upper_bound to find the latest time <= query. set/get O(log n).
#include <bits/stdc++.h>
using namespace std;

struct TimeMap {
    unordered_map<int, map<int, int>> store; // key -> (time -> value)
    void set(int key, int value, int time) { store[key][time] = value; }
    // returns {found, value}
    pair<bool, int> get(int key, int time) {
        auto it = store.find(key);
        if (it == store.end()) return {false, 0};
        auto& m = it->second;
        auto up = m.upper_bound(time); // first time > query
        if (up == m.begin()) return {false, 0};
        return {true, prev(up)->second};
    }
};

void show(pair<bool, int> r) { cout << (r.first ? to_string(r.second) : "null") << "\n"; }

int main() {
    // The README's three blocks are independent scenarios (fresh maps).
    TimeMap a;
    a.set(1, 1, 0); a.set(1, 2, 2);
    show(a.get(1, 1)); // 1
    show(a.get(1, 3)); // 2

    TimeMap b;
    b.set(1, 1, 5);
    show(b.get(1, 0)); // null
    show(b.get(1, 10)); // 1

    TimeMap c;
    c.set(1, 1, 0); c.set(1, 2, 0);
    show(c.get(1, 0)); // 2
    return 0;
}
