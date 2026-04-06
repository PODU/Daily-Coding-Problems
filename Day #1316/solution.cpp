// Smallest sparse number (no adjacent 1 bits) >= N. Scan bits low->high; at the
// top of each adjacent-ones run, carry up and clear below. Time O(log N).
#include <bits/stdc++.h>
using namespace std;

long long nextSparse(long long x) {
    if (x == 0) return 0;
    vector<int> b;
    for (long long t = x; t; t >>= 1) b.push_back(t & 1);
    b.push_back(0); b.push_back(0); // padding for carries
    int n = b.size();
    for (int i = 1; i < n - 1; i++) {
        if (b[i] == 1 && b[i - 1] == 1 && b[i + 1] == 0) {
            b[i + 1] = 1;
            for (int j = i; j >= 0; j--) b[j] = 0;
        }
    }
    long long ans = 0;
    for (int i = 0; i < n; i++) if (b[i]) ans |= (1LL << i);
    return ans;
}

int main() {
    cout << nextSparse(22) << "\n"; // 32
    return 0;
}
