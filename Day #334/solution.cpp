// 24 Game (fixed order): recursive split of contiguous list, combine with +,-,*,/ (doubles).
// Time: O(1) for fixed 4 numbers. Space: O(1).
#include <bits/stdc++.h>
using namespace std;

vector<double> solve(const vector<double>& a) {
    int n = a.size();
    if (n == 1) return {a[0]};
    vector<double> res;
    for (int i = 1; i < n; ++i) {
        vector<double> L = solve(vector<double>(a.begin(), a.begin() + i));
        vector<double> R = solve(vector<double>(a.begin() + i, a.end()));
        for (double l : L) for (double r : R) {
            res.push_back(l + r);
            res.push_back(l - r);
            res.push_back(l * r);
            if (fabs(r) > 1e-9) res.push_back(l / r);
        }
    }
    return res;
}

bool can24(const vector<double>& a) {
    for (double v : solve(a)) if (fabs(v - 24.0) < 1e-6) return true;
    return false;
}

int main() {
    vector<double> nums = {5, 2, 7, 8};
    cout << (can24(nums) ? "True" : "False") << endl;
    return 0;
}
