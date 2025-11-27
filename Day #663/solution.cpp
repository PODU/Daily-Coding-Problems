// Day 663: HitCounter. Keep timestamps sorted; total = O(1); range = binary search.
// record O(log n) amortized (sorted insert), total O(1), range O(log n).
// Limited-memory follow-up: bucket hits into fixed time windows and store (window,count).
#include <bits/stdc++.h>
using namespace std;

struct HitCounter {
    multiset<long long> ts;
    void record(long long t) { ts.insert(t); }
    long long total() { return (long long)ts.size(); }
    long long range(long long lo, long long hi) {
        return distance(ts.lower_bound(lo), ts.upper_bound(hi));
    }
};

int main() {
    HitCounter h;
    for (long long t : {1, 2, 2, 5, 9, 10}) h.record(t);
    cout << "total: " << h.total() << "\n";        // 6
    cout << "range(2,9): " << h.range(2, 9) << "\n"; // 4 (2,2,5,9)
    return 0;
}
