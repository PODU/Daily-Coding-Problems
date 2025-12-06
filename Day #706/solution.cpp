// Day 706: 24 Game (fixed order). Try every parenthesization (Catalan) over the
// fixed sequence, combining sub-results with +,-,*,/. Time ~O(1) for 4 numbers, O(n^3 * states) general.
#include <bits/stdc++.h>
using namespace std;

vector<double> solve(const vector<double>& nums) {
    int n = nums.size();
    if (n == 1) return {nums[0]};
    vector<double> res;
    for (int i = 1; i < n; ++i) {
        vector<double> L(nums.begin(), nums.begin() + i);
        vector<double> R(nums.begin() + i, nums.end());
        auto lv = solve(L), rv = solve(R);
        for (double a : lv) for (double b : rv) {
            res.push_back(a + b);
            res.push_back(a - b);
            res.push_back(a * b);
            if (fabs(b) > 1e-9) res.push_back(a / b);
        }
    }
    return res;
}

bool game24(const vector<int>& digits) {
    vector<double> nums(digits.begin(), digits.end());
    for (double v : solve(nums))
        if (fabs(v - 24.0) < 1e-6) return true;
    return false;
}

int main() {
    vector<int> input = {5, 2, 7, 8};
    cout << (game24(input) ? "True" : "False") << endl;
    return 0;
}
