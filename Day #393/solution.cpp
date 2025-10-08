// Largest consecutive range via hash set: from each start (n-1 absent) extend up. O(n) time, O(n) space.
#include <vector>
#include <unordered_set>
#include <iostream>
using namespace std;

pair<int,int> largestRange(const vector<int>& nums) {
    unordered_set<int> s(nums.begin(), nums.end());
    int bestLo = nums[0], bestHi = nums[0], bestLen = 0;
    for (int n : s) {
        if (s.count(n - 1)) continue;
        int hi = n;
        while (s.count(hi + 1)) ++hi;
        if (hi - n + 1 > bestLen) { bestLen = hi - n + 1; bestLo = n; bestHi = hi; }
    }
    return {bestLo, bestHi};
}

int main() {
    vector<int> nums = {9, 6, 1, 3, 8, 10, 12, 11};
    auto r = largestRange(nums);
    cout << "(" << r.first << ", " << r.second << ")" << endl;
    return 0;
}
