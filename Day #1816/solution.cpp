// Time-versioned map: per key keep times sorted; get returns value at most-recent time <= t.
// set O(log m) (map), get O(log m) via upper_bound. Space: O(total entries).
#include <bits/stdc++.h>
using namespace std;

class TimeMap {
    unordered_map<int, map<int,int>> data; // key -> (time -> value)
public:
    void set(int key, int value, int time) { data[key][time] = value; }
    // returns {found, value}
    pair<bool,int> get(int key, int time) {
        auto it = data.find(key);
        if (it == data.end()) return {false, 0};
        auto& m = it->second;
        auto up = m.upper_bound(time); // first time > query
        if (up == m.begin()) return {false, 0};
        --up;
        return {true, up->second};
    }
};

void show(TimeMap& d, int key, int time) {
    auto r = d.get(key, time);
    cout << "get(" << key << ", " << time << ") = " << (r.first ? to_string(r.second) : "null") << "\n";
}

int main() {
    { TimeMap d; d.set(1,1,0); d.set(1,2,2); show(d,1,1); show(d,1,3); } // 1, 2
    { TimeMap d; d.set(1,1,5); show(d,1,0); show(d,1,10); }              // null, 1
    { TimeMap d; d.set(1,1,0); d.set(1,2,0); show(d,1,0); }              // 2
    return 0;
}
