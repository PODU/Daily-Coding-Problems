// Count smaller elements to the right via Fenwick tree + coordinate compression.
// Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct BIT {
    vector<int> t;
    BIT(int n) : t(n + 1, 0) {}
    void update(int i) { for (; i < (int)t.size(); i += i & -i) t[i]++; }
    int query(int i) { int s = 0; for (; i > 0; i -= i & -i) s += t[i]; return s; }
};

vector<int> countSmaller(const vector<int>& a) {
    int n = a.size();
    vector<int> sorted_a(a);
    sort(sorted_a.begin(), sorted_a.end());
    sorted_a.erase(unique(sorted_a.begin(), sorted_a.end()), sorted_a.end());
    BIT bit(sorted_a.size());
    vector<int> res(n);
    for (int i = n - 1; i >= 0; i--) {
        int rank = lower_bound(sorted_a.begin(), sorted_a.end(), a[i]) - sorted_a.begin() + 1;
        res[i] = bit.query(rank - 1);
        bit.update(rank);
    }
    return res;
}

int main() {
    vector<int> a = {3, 4, 9, 6, 1};
    vector<int> res = countSmaller(a);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";
}
