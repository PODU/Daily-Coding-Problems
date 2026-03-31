// Day 1283: For each element, count smaller elements to its right.
// Fenwick (BIT) over compressed values, scanning right-to-left. Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct BIT {
    vector<int> t;
    BIT(int n) : t(n + 1, 0) {}
    void update(int i, int v) { for (; i < (int)t.size(); i += i & -i) t[i] += v; }
    int query(int i) { int s = 0; for (; i > 0; i -= i & -i) s += t[i]; return s; }
};

vector<int> countSmaller(const vector<int>& a) {
    int n = a.size();
    vector<int> sorted(a);
    sort(sorted.begin(), sorted.end());
    sorted.erase(unique(sorted.begin(), sorted.end()), sorted.end());
    BIT bit(sorted.size());
    vector<int> res(n);
    for (int i = n - 1; i >= 0; --i) {
        int rank = lower_bound(sorted.begin(), sorted.end(), a[i]) - sorted.begin() + 1;
        res[i] = bit.query(rank - 1);
        bit.update(rank, 1);
    }
    return res;
}

int main() {
    vector<int> a = {3, 4, 9, 6, 1};
    auto r = countSmaller(a);
    cout << "[";
    for (size_t i = 0; i < r.size(); ++i) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // [1, 1, 2, 1, 0]
    return 0;
}
