// Quxes: adjacent different colors merge to third. Smallest remaining count.
// Count r,g,b; distinct<=1 -> total; all same parity -> 2; else 1. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int smallestQuxes(const vector<char>& q) {
    int r = 0, g = 0, b = 0;
    for (char c : q) {
        if (c == 'R') r++;
        else if (c == 'G') g++;
        else b++;
    }
    int distinct = (r > 0) + (g > 0) + (b > 0);
    if (distinct <= 1) return r + g + b;
    if (r % 2 == g % 2 && g % 2 == b % 2) return 2;
    return 1;
}

int main() {
    vector<char> q = {'R', 'G', 'B', 'G', 'B'};
    cout << smallestQuxes(q) << endl;
    return 0;
}
