// Day 424: Two unique elements via XOR partition. O(n) time, O(1) space.
// XOR all -> a^b; isolate a low set bit; partition & XOR each group -> a, b.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<long long> a = {2, 4, 6, 8, 10, 2, 6, 10};
    long long x = 0;
    for (long long v : a) x ^= v;
    long long bit = x & (-x);
    long long p = 0, q = 0;
    for (long long v : a) {
        if (v & bit) p ^= v;
        else q ^= v;
    }
    long long lo = min(p, q), hi = max(p, q);
    cout << lo << " and " << hi << "\n";
    return 0;
}
