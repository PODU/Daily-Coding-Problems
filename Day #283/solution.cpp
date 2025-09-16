// Day 283: First N regular (5-smooth) numbers via 3-pointer merge of 2x,3x,5x.
// Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

vector<long long> regular(int n) {
    vector<long long> h(n);
    h[0] = 1;
    int i2 = 0, i3 = 0, i5 = 0;
    for (int i = 1; i < n; i++) {
        long long n2 = h[i2] * 2, n3 = h[i3] * 3, n5 = h[i5] * 5;
        long long nxt = min({n2, n3, n5});
        h[i] = nxt;
        if (nxt == n2) i2++;
        if (nxt == n3) i3++;
        if (nxt == n5) i5++;
    }
    return h;
}

int main() {
    for (long long x : regular(10)) cout << x << " "; // 1 2 3 4 5 6 8 9 10 12
    cout << "\n";
    return 0;
}
