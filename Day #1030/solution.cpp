// Day 1030: Quxes minimum remaining. Count colors; parity-based O(N) formula.
// If two counts are 0 -> n; else if all parities equal -> 2; else -> 1. Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minQuxes(const vector<char>& q) {
    int r = 0, g = 0, b = 0;
    for (char c : q) { if (c == 'R') r++; else if (c == 'G') g++; else b++; }
    int n = r + g + b;
    int zeros = (r == 0) + (g == 0) + (b == 0);
    if (zeros >= 2) return n;                 // all same color, cannot reduce
    if ((r % 2 == g % 2) && (g % 2 == b % 2)) return 2;
    return 1;
}

int main() {
    vector<char> q = {'R', 'G', 'B', 'G', 'B'};
    cout << minQuxes(q) << "\n";
    return 0;
}
