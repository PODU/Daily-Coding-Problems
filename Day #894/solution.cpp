// HitCounter: maintain timestamps in a sorted multiset. record = sorted insert (O(log n)),
// total = size O(1), range = upper_bound - lower_bound via binary search O(log n).
// Limited-memory follow-up: bucket hits by time window (circular buffer of fixed size)
// so memory stays O(window) instead of O(#hits), trading exact old-range queries for recency.
#include <bits/stdc++.h>
using namespace std;

class HitCounter {
    multiset<long long> ts;
public:
    void record(long long t) { ts.insert(t); }
    int total() const { return (int)ts.size(); }
    int range(long long lower, long long upper) const {
        return (int)distance(ts.lower_bound(lower), ts.upper_bound(upper));
    }
};

int main() {
    HitCounter hc;
    hc.record(1);
    hc.record(2);
    hc.record(3);
    hc.record(2);
    cout << hc.total() << endl;        // 4
    cout << hc.range(2, 3) << endl;    // 3
    return 0;
}
