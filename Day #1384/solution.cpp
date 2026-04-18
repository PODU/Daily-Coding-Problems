// HitCounter: keep timestamps in a sorted multiset; record O(log n), total O(1) via counter, range via binary search (upper-lower).
#include <bits/stdc++.h>
using namespace std;

class HitCounter {
    multiset<int> hits;
    long long cnt = 0;
public:
    void record(int t) { hits.insert(t); cnt++; }
    long long total() const { return cnt; }
    long long range(int lo, int hi) const {
        return distance(hits.lower_bound(lo), hits.upper_bound(hi));
    }
};

int main() {
    HitCounter hc;
    for (int t : {1, 1, 2, 3, 5, 8}) hc.record(t);
    cout << "total: " << hc.total() << "\n";
    cout << "range(2,5): " << hc.range(2, 5) << "\n";
    return 0;
}
