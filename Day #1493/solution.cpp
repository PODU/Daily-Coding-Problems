// Detect a Pythagorean triplet a^2+b^2=c^2 in an array.
// Square + sort, then fix largest as c and two-pointer. Time O(n^2), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool hasTriplet(vector<long long> nums) {
    for (auto& x : nums) x *= x;
    sort(nums.begin(), nums.end());
    int n = nums.size();
    for (int c = n - 1; c >= 2; --c) {
        int a = 0, b = c - 1;
        while (a < b) {
            long long s = nums[a] + nums[b];
            if (s == nums[c]) return true;
            if (s < nums[c]) ++a; else --b;
        }
    }
    return false;
}

int main() {
    vector<long long> nums = {3, 1, 4, 6, 5};
    cout << (hasTriplet(nums) ? "true" : "false") << endl; // expected: true
    return 0;
}
