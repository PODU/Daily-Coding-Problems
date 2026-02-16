// XOR all -> a^b; isolate via lowest set bit; partition & XOR each group to recover a,b; O(n) time O(1) space
#include <bits/stdc++.h>
using namespace std;

pair<int,int> findTwo(vector<int>& nums) {
    int xorAll = 0;
    for (int n : nums) xorAll ^= n;
    int bit = xorAll & (-xorAll);          // lowest set bit differs between a and b
    int a = 0, b = 0;
    for (int n : nums) {
        if (n & bit) a ^= n;
        else         b ^= n;
    }
    return {min(a, b), max(a, b)};
}

int main() {
    vector<int> nums = {2, 4, 6, 8, 10, 2, 6, 10};
    auto [a, b] = findTwo(nums);
    cout << a << " and " << b << "\n";
    return 0;
}
