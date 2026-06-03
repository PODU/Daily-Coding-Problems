// Subscribers-per-hour over 24 hours via Fenwick/BIT. update(hour,val)+=, query(start,end)=inclusive range sum.
// Time O(log n) per op, Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Fenwick {
    int n;
    vector<long long> tree;
    Fenwick(int n) : n(n), tree(n + 1, 0) {}
    void add(int i, long long v) { for (++i; i <= n; i += i & -i) tree[i] += v; }
    long long pref(int i) { long long s = 0; for (++i; i > 0; i -= i & -i) s += tree[i]; return s; }
    long long range(int l, int r) { return pref(r) - (l ? pref(l - 1) : 0); }
};

int main() {
    Fenwick bit(24);            // all zeros
    bit.add(0, 5);
    bit.add(3, 10);
    bit.add(23, 2);
    cout << bit.range(0, 23) << "\n"; // 17
    cout << bit.range(0, 3) << "\n";  // 15
    cout << bit.range(1, 2) << "\n";  // 0
    return 0;
}
