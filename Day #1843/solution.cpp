// Day 1843: Max of two numbers with no branching/comparison; uses sign bit of the difference.
// max(a,b) = a - ((a-b) & ((a-b) >> 63)). Time O(1), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long maxNoBranch(long long a, long long b) {
    long long d = a - b;
    // d>>63 is 0 when a>=b, all-ones (-1) when a<b
    return a - (d & (d >> 63));
}

int main() {
    cout << maxNoBranch(5, 9) << "\n";   // 9
    cout << maxNoBranch(12, 4) << "\n";  // 12
    cout << maxNoBranch(-3, -7) << "\n"; // -3
}
