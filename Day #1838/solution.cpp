// Day 1838: Count smaller elements to the right, via a Fenwick tree over compressed values.
// Time O(N log N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

struct BIT {
    vector<int> t;
    BIT(int n) : t(n + 1, 0) {}
    void add(int i) { for (; i < (int)t.size(); i += i & -i) t[i]++; }
    int sum(int i) { int s = 0; for (; i > 0; i -= i & -i) s += t[i]; return s; }
};

vector<int> countSmaller(const vector<int>& a) {
    int n = a.size();
    vector<int> sorted(a.begin(), a.end());
    sort(sorted.begin(), sorted.end());
    sorted.erase(unique(sorted.begin(), sorted.end()), sorted.end());
    BIT bit(sorted.size());
    vector<int> res(n);
    for (int i = n - 1; i >= 0; i--) {
        int r = lower_bound(sorted.begin(), sorted.end(), a[i]) - sorted.begin() + 1; // 1-indexed
        res[i] = bit.sum(r - 1);  // count strictly smaller seen so far
        bit.add(r);
    }
    return res;
}

int main() {
    vector<int> a = {3, 4, 9, 6, 1};
    auto res = countSmaller(a);
    cout << "[";
    for (int i = 0; i < (int)res.size(); i++) cout << res[i] << (i + 1 < (int)res.size() ? ", " : "");
    cout << "]\n";
}
