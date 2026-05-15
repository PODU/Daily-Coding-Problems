// Single number among triples: bitwise ones/twos accumulators isolate the unique value.
// Time: O(n). Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int singleNumber(const vector<int>& nums) {
    int ones = 0, twos = 0;
    for (int x : nums) {
        ones = (ones ^ x) & ~twos;
        twos = (twos ^ x) & ~ones;
    }
    return ones;
}

int main() {
    vector<int> a = {6, 1, 3, 3, 3, 6, 6};
    vector<int> b = {13, 19, 13, 13};
    cout << singleNumber(a) << "\n"; // 1
    cout << singleNumber(b) << "\n"; // 19
    return 0;
}
