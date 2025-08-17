// XOR all -> a^b; isolate a differing bit; partition into two groups and XOR each.
// Time O(n); Space O(1).
#include <iostream>
#include <vector>
using namespace std;

pair<int, int> twoSingles(const vector<int>& nums) {
    long long x = 0;
    for (int v : nums) x ^= v;
    int bit = x & (-x);              // lowest set bit where the two singles differ
    int a = 0, b = 0;
    for (int v : nums) {
        if (v & bit) a ^= v;
        else b ^= v;
    }
    if (a > b) swap(a, b);
    return {a, b};
}

int main() {
    vector<int> nums = {2, 4, 6, 8, 10, 2, 6, 10};
    auto r = twoSingles(nums);
    cout << r.first << " and " << r.second << endl; // 4 and 8
    return 0;
}
