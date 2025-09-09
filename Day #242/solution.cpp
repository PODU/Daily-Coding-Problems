// Fenwick/Binary Indexed Tree over 24 hours.
// update: O(log n), query (prefix-diff): O(log n). Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct BIT {
    int n;
    vector<long long> tree;
    BIT(int n) : n(n), tree(n + 1, 0) {}
    void add(int i, long long v) {          // 0-based index
        for (++i; i <= n; i += i & (-i)) tree[i] += v;
    }
    long long prefix(int i) {               // sum of [0..i], 0-based
        long long s = 0;
        for (++i; i > 0; i -= i & (-i)) s += tree[i];
        return s;
    }
    long long query(int l, int r) {         // inclusive [l..r]
        return prefix(r) - (l > 0 ? prefix(l - 1) : 0);
    }
    void update(int hour, long long value) { add(hour, value); }
};

int main() {
    BIT bit(24);
    bit.update(2, 5);
    bit.update(5, 3);
    bit.update(23, 10);
    cout << "query(2,5) = " << bit.query(2, 5) << "\n";
    cout << "query(0,23) = " << bit.query(0, 23) << "\n";
    return 0;
}
