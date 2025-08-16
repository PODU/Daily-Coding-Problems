// Day 132: HitCounter (record, total, range).
// Keep timestamps sorted; range via binary search. record O(n) insert, total O(1), range O(log n).
// Limited-memory follow-up: bucket counts by coarse time granularity instead of per-hit.
#include <bits/stdc++.h>
using namespace std;

struct HitCounter {
    vector<long long> ts; // sorted
    void record(long long t) {
        ts.insert(upper_bound(ts.begin(), ts.end(), t), t);
    }
    int total() { return ts.size(); }
    int range(long long lo, long long hi) {
        auto a = lower_bound(ts.begin(), ts.end(), lo);
        auto b = upper_bound(ts.begin(), ts.end(), hi);
        return b - a;
    }
};

int main() {
    HitCounter hc;
    for (long long t : {1, 1, 2, 3, 5, 8, 8, 10}) hc.record(t);
    cout << "total = " << hc.total() << endl;          // 8
    cout << "range(2, 8) = " << hc.range(2, 8) << endl; // 5
    return 0;
}
