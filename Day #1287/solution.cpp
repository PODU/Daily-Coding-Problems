// Day 1287: Minimum bonuses so each employee out-earns lower-scoring neighbors.
// Two passes (left->right, right->left), take max. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

vector<int> bonuses(const vector<int>& a) {
    int n = a.size();
    vector<int> b(n, 1);
    for (int i = 1; i < n; ++i)
        if (a[i] > a[i - 1]) b[i] = b[i - 1] + 1;
    for (int i = n - 2; i >= 0; --i)
        if (a[i] > a[i + 1]) b[i] = max(b[i], b[i + 1] + 1);
    return b;
}

int main() {
    auto r = bonuses({10, 40, 200, 1000, 60, 30});
    cout << "[";
    for (size_t i = 0; i < r.size(); ++i) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // [1, 2, 3, 4, 2, 1]
    return 0;
}
