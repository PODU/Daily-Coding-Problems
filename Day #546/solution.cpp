// Count smaller elements to the right via Fenwick/BIT + coordinate compression.
// Time O(n log n), Space O(n).
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

struct BIT {
    vector<int> t;
    BIT(int n) : t(n + 1, 0) {}
    void update(int i, int v) { for (; i < (int)t.size(); i += i & -i) t[i] += v; }
    int query(int i) { int s = 0; for (; i > 0; i -= i & -i) s += t[i]; return s; }
};

vector<int> countSmaller(const vector<int>& a) {
    int n = a.size();
    vector<int> sorted(a.begin(), a.end());
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
    vector<int> res = countSmaller(a);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << res[i];
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
