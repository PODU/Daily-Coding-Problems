// Day 766: Minimum flips so all 'x' come before all 'y'.
// One-pass DP: flips = min(flips+1, countY). O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int minFlips(const string& s) {
    int flips = 0, countY = 0;
    for (char c : s) {
        if (c == 'y') countY++;
        else flips = min(flips + 1, countY);
    }
    return flips;
}

int main() {
    cout << minFlips("xyxxxyxyy") << endl; // 2
    return 0;
}
