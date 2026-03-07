// 24 game (fixed order): interval DP/recursion over the sequence, combining
// left/right reachable values with + - * / using doubles + epsilon.
// Time: O(n^3 * S^2) over splits/value-sets, Space: O(n^2 * S). Here n=4.
#include <bits/stdc++.h>
using namespace std;

vector<double> solve(const vector<int>& a, int l, int r) {
    if (l == r) return {(double)a[l]};
    vector<double> res;
    for (int m = l; m < r; ++m) {
        auto L = solve(a, l, m);
        auto R = solve(a, m + 1, r);
        for (double x : L) for (double y : R) {
            res.push_back(x + y);
            res.push_back(x - y);
            res.push_back(x * y);
            if (fabs(y) > 1e-9) res.push_back(x / y);
        }
    }
    return res;
}

bool can24(const vector<int>& a) {
    auto vals = solve(a, 0, (int)a.size() - 1);
    for (double v : vals) if (fabs(v - 24.0) < 1e-6) return true;
    return false;
}

int main() {
    vector<int> a = {5, 2, 7, 8};
    cout << (can24(a) ? "True" : "False") << endl;
    return 0;
}
