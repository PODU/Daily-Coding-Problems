// Day 433: Next larger integer with the same number of set bits (Gosper's hack).
// c = n & -n; r = n + c; next = (((r ^ n) >> 2) / c) | r. O(1) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

long long nextSameBits(long long n) {
    if (n <= 0) return n;
    long long c = n & (-n);
    long long r = n + c;
    return (((r ^ n) >> 2) / c) | r;
}

string toBin(long long n) {
    if (n == 0) return "0";
    string s;
    while (n) { s += char('0' + (n & 1)); n >>= 1; }
    reverse(s.begin(), s.end());
    return s;
}

int main() {
    long long n = 6, m = nextSameBits(n);
    cout << "Input: " << n << " (" << toBin(n) << " in binary)\n";
    cout << "Next: " << m << " (" << toBin(m) << " in binary)\n";
}
