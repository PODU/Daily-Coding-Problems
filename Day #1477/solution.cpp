// Day 1477: First N regular numbers (only prime factors 2, 3, 5).
// DP with three pointers merging *2, *3, *5 sequences. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

vector<long long> regularNumbers(int n) {
    if (n <= 0) return {};
    vector<long long> h(n, 1);
    int i2 = 0, i3 = 0, i5 = 0;
    for (int k = 1; k < n; ++k) {
        long long nxt = min({h[i2] * 2, h[i3] * 3, h[i5] * 5});
        h[k] = nxt;
        if (nxt == h[i2] * 2) ++i2;
        if (nxt == h[i3] * 3) ++i3;
        if (nxt == h[i5] * 5) ++i5;
    }
    return h;
}

int main() {
    auto r = regularNumbers(10);
    cout << "[";
    for (size_t i = 0; i < r.size(); ++i) cout << (i ? ", " : "") << r[i];
    cout << "]\n";  // [1, 2, 3, 4, 5, 6, 8, 9, 10, 12]
}
