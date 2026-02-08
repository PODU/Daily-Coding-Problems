// Time-versioned map: per key a std::map<time,value>; get uses upper_bound for floor.
// set/get O(log n). Setting same key+time overwrites.
#include <bits/stdc++.h>
using namespace std;

struct TimeMap {
    unordered_map<int, map<int, int>> store;
    void set(int key, int value, int time) { store[key][time] = value; }
    // returns {found, value}
    pair<bool, int> get(int key, int time) {
        auto it = store.find(key);
        if (it == store.end()) return {false, 0};
        auto& m = it->second;
        auto ub = m.upper_bound(time);
        if (ub == m.begin()) return {false, 0};
        --ub;
        return {true, ub->second};
    }
};

static string show(pair<bool, int> r) {
    return r.first ? to_string(r.second) : "null";
}

int main() {
    // README presents three logical blocks; each starts from its own map state.
    TimeMap d;
    d.set(1, 1, 0);
    d.set(1, 2, 2);
    cout << "d.get(1, 1) -> " << show(d.get(1, 1)) << "\n";
    cout << "d.get(1, 3) -> " << show(d.get(1, 3)) << "\n";

    d = TimeMap();
    d.set(1, 1, 5);
    cout << "d.get(1, 0) -> " << show(d.get(1, 0)) << "\n";
    cout << "d.get(1, 10) -> " << show(d.get(1, 10)) << "\n";

    d = TimeMap();
    d.set(1, 1, 0);
    d.set(1, 2, 0); // overwrite same key+time -> value 2
    cout << "d.get(1, 0) -> " << show(d.get(1, 0)) << "\n";
    return 0;
}
