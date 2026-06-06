// Day 1621: First N regular (5-smooth/Hamming) numbers.
// DP merge with 3 pointers for factors 2,3,5. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

vector<long long> regularNumbers(int n) {
    vector<long long> h(n);
    h[0] = 1;
    int i2 = 0, i3 = 0, i5 = 0;
    for (int i = 1; i < n; i++) {
        long long n2 = h[i2] * 2, n3 = h[i3] * 3, n5 = h[i5] * 5;
        long long m = min({n2, n3, n5});
        h[i] = m;
        if (m == n2) i2++;
        if (m == n3) i3++;
        if (m == n5) i5++;
    }
    return h;
}

int main() {
    auto r = regularNumbers(10);
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? " " : "\n");
    return 0;
}
