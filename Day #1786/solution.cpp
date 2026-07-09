// Recursively combine adjacent pairs (preserves order, covers all parenthesizations)
// applying +,-,*,/ until one value remains; check |v-24|<1e-6.
// Time O(exponential in n), Space O(n) recursion. Here n=4.
#include <iostream>
#include <vector>
#include <cmath>
using namespace std;

bool solve(vector<double> nums) {
    if (nums.size() == 1) return fabs(nums[0] - 24.0) < 1e-6;
    for (size_t i = 0; i + 1 < nums.size(); i++) {
        double a = nums[i], b = nums[i+1];
        vector<double> results = {a+b, a-b, a*b};
        if (fabs(b) > 1e-9) results.push_back(a/b);
        for (double r : results) {
            vector<double> next;
            for (size_t j = 0; j < nums.size(); j++) {
                if (j == i) next.push_back(r);
                else if (j == i+1) continue;
                else next.push_back(nums[j]);
            }
            if (solve(next)) return true;
        }
    }
    return false;
}

bool canGet24(vector<int> nums) {
    vector<double> d(nums.begin(), nums.end());
    return solve(d);
}

int main() {
    cout << (canGet24({5,2,7,8}) ? "True" : "False") << "\n";
    return 0;
}
