// Day 662: GCD of n numbers via repeated Euclidean algorithm.
// Time O(n * log(max)), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int gcd2(int a, int b) { while (b) { int t = a % b; a = b; b = t; } return a; }

int gcdAll(const vector<int>& v) {
    int g = 0;
    for (int x : v) g = gcd2(g, x);
    return g;
}

int main() {
    vector<int> v = {42, 56, 14};
    cout << gcdAll(v) << "\n"; // 14
    return 0;
}
