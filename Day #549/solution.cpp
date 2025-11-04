// Single number among triples: sum each bit position mod 3 to rebuild the unique value. O(N) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int singleNumber(const vector<int>& nums) {
    int result = 0;
    for (int b = 0; b < 32; ++b) {
        int cnt = 0;
        for (int x : nums) cnt += (x >> b) & 1;
        if (cnt % 3) result |= (1 << b);
    }
    return result;
}

int main() {
    cout << singleNumber({6, 1, 3, 3, 3, 6, 6}) << "\n";
    cout << singleNumber({13, 19, 13, 13}) << "\n";
    return 0;
}
