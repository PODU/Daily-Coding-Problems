// Min Quxes remaining. One color -> N; all counts same parity -> 2; else 1.
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minQuxes(const string& q) {
    if (q.empty()) return 0;
    int r = 0, g = 0, b = 0;
    for (char c : q) { if (c == 'R') ++r; else if (c == 'G') ++g; else ++b; }
    int distinct = (r > 0) + (g > 0) + (b > 0);
    if (distinct == 1) return (int)q.size();
    if (r % 2 == g % 2 && g % 2 == b % 2) return 2;
    return 1;
}

int main() {
    cout << minQuxes("RGBGB") << "\n";
    return 0;
}
