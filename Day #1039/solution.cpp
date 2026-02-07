// Fenwick/BIT over 24 hours: point update, prefix-sum range query.
// update O(log n), query O(log n).
#include <bits/stdc++.h>
using namespace std;

struct BIT {
    int n;
    vector<long long> tree;
    BIT(int n) : n(n), tree(n + 1, 0) {}
    void update(int hour, long long value) {
        for (int i = hour + 1; i <= n; i += i & (-i)) tree[i] += value;
    }
    long long prefix(int idx) { // sum of [0..idx]
        long long s = 0;
        for (int i = idx + 1; i > 0; i -= i & (-i)) s += tree[i];
        return s;
    }
    long long query(int start, int end) { // inclusive
        return prefix(end) - (start > 0 ? prefix(start - 1) : 0);
    }
};

int main() {
    BIT bit(24);
    bit.update(0, 5);
    bit.update(3, 10);
    bit.update(23, 2);
    bit.update(3, 1);
    cout << "query(0, 3) = " << bit.query(0, 3) << "\n";
    cout << "query(0, 23) = " << bit.query(0, 23) << "\n";
    cout << "query(4, 23) = " << bit.query(4, 23) << "\n";
    cout << "query(3, 3) = " << bit.query(3, 3) << "\n";
    return 0;
}
