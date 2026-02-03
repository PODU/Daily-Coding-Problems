// Branchless max: use 64-bit diff to avoid overflow; sign bit via arithmetic
// right shift selects the larger. No if/branch/compare. Time O(1), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int maxNoBranch(int a, int b) {
    long long diff = (long long)a - (long long)b; // a - b without int overflow
    // sign = -1 (all ones) if diff < 0, else 0
    long long sign = diff >> 63; // arithmetic right shift of 64-bit
    // if a>=b: sign=0 -> a; if a<b: sign=-1 -> b
    return (int)(a - (diff & sign));
}

int main() {
    cout << "max(3, 7) = " << maxNoBranch(3, 7) << "\n";
    cout << "max(10, -5) = " << maxNoBranch(10, -5) << "\n";
    return 0;
}
