// Day 1756: HitCounter design.
// Store timestamps in a sorted vector; total() O(1), range() via binary search O(log n).
// Limited-memory follow-up: bucket hits by coarse time granularity (e.g. per-second
// counts in a map/ring buffer) so memory is O(#buckets) instead of O(#hits).
#include <bits/stdc++.h>
using namespace std;

class HitCounter {
    vector<long long> hits; // kept sorted (assumes records arrive in order)
public:
    void record(long long timestamp) {
        // If out of order, insert at correct position to keep sorted.
        if (!hits.empty() && timestamp < hits.back()) {
            auto it = upper_bound(hits.begin(), hits.end(), timestamp);
            hits.insert(it, timestamp);
        } else {
            hits.push_back(timestamp);
        }
    }
    int total() const { return (int)hits.size(); }
    int range(long long lower, long long upper) const {
        auto lo = lower_bound(hits.begin(), hits.end(), lower);
        auto hi = upper_bound(hits.begin(), hits.end(), upper);
        return (int)(hi - lo);
    }
};

int main() {
    HitCounter hc;
    for (long long t : {1, 2, 2, 5, 7, 9, 10}) hc.record(t);

    cout << "total() = " << hc.total() << endl;           // 7
    cout << "range(2, 7) = " << hc.range(2, 7) << endl;   // 4 (2,2,5,7)
    cout << "range(0, 10) = " << hc.range(0, 10) << endl; // 7
    return 0;
}
