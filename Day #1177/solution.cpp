// Day 1177: Find the element appearing once while all others appear 3 times.
// Track bits seen once (ones) and twice (twos); a third sighting clears both.
// Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int singleNumber(const vector<int>& a) {
    int ones = 0, twos = 0;
    for (int x : a) {
        ones = (ones ^ x) & ~twos;
        twos = (twos ^ x) & ~ones;
    }
    return ones;
}

int main() {
    cout << singleNumber({6, 1, 3, 3, 3, 6, 6}) << "\n"; // 1
    cout << singleNumber({13, 19, 13, 13}) << "\n";       // 19
    return 0;
}
