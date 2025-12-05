// Day 696: 24-hour subscriber array with point update + inclusive range-sum query.
// Approach: Fenwick (Binary Indexed) Tree. update O(log n), query O(log n), space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Fenwick {
    vector<long long> t;
    int n;
    Fenwick(int n) : t(n + 1, 0), n(n) {}
    void add(int i, long long v) { for (++i; i <= n; i += i & -i) t[i] += v; }
    long long pref(int i) { long long s = 0; for (++i; i > 0; i -= i & -i) s += t[i]; return s; }
    long long range(int l, int r) { return pref(r) - (l ? pref(l - 1) : 0); }
};

struct Subscribers {
    Fenwick f;
    Subscribers() : f(24) {}
    void update(int hour, int value) { f.add(hour, value); }
    long long query(int start, int end) { return f.range(start, end); }
};

int main() {
    Subscribers s;
    s.update(3, 10);
    s.update(5, 7);
    s.update(10, 4);
    cout << s.query(3, 10) << "\n";  // 21
    cout << s.query(0, 4) << "\n";   // 10
    s.update(3, 5);
    cout << s.query(3, 10) << "\n";  // 26
    return 0;
}
