// Day 1489: Time-indexed map. Per key, keep entries sorted by time; get does
// binary search for the most recent time <= query. set O(log n) amortized, get O(log n).
#include <iostream>
#include <map>
#include <vector>
#include <algorithm>
#include <climits>
using namespace std;

class TimeMap {
    // key -> sorted list of (time, value). Same time => later set overwrites.
    map<int, vector<pair<int,int>>> store;
public:
    void set(int key, int value, int time) {
        auto& v = store[key];
        auto it = lower_bound(v.begin(), v.end(), make_pair(time, INT_MIN),
                              [](const pair<int,int>& a, const pair<int,int>& b){
                                  return a.first < b.first; });
        if (it != v.end() && it->first == time) it->second = value; // overwrite same time
        else v.insert(it, {time, value});
    }
    // returns true + value, or false if nothing set at/before time.
    bool get(int key, int time, int& outVal) {
        auto kit = store.find(key);
        if (kit == store.end()) return false;
        auto& v = kit->second;
        // rightmost entry with first <= time
        int lo = 0, hi = (int)v.size() - 1, idx = -1;
        while (lo <= hi) {
            int mid = (lo + hi) / 2;
            if (v[mid].first <= time) { idx = mid; lo = mid + 1; }
            else hi = mid - 1;
        }
        if (idx < 0) return false;
        outVal = v[idx].second;
        return true;
    }
};

static void show(TimeMap& d, int key, int time) {
    int val;
    if (d.get(key, time, val)) cout << "get(" << key << "," << time << ") = " << val << "\n";
    else cout << "get(" << key << "," << time << ") = null\n";
}

int main() {
    TimeMap d1;
    d1.set(1, 1, 0); d1.set(1, 2, 2);
    show(d1, 1, 1);   // 1
    show(d1, 1, 3);   // 2

    TimeMap d2;
    d2.set(1, 1, 5);
    show(d2, 1, 0);   // null
    show(d2, 1, 10);  // 1

    TimeMap d3;
    d3.set(1, 1, 0); d3.set(1, 2, 0);
    show(d3, 1, 0);   // 2
    return 0;
}
