// Day 1753: Egyptian fraction (sum of distinct unit fractions).
// Greedy Fibonacci-Sylvester: repeatedly subtract 1/ceil(b/a). Time O(terms), O(1) space.
#include <bits/stdc++.h>
using namespace std;

string egyptian(long long a, long long b) {
    string out;
    bool first = true;
    while (a != 0) {
        long long c = (b + a - 1) / a; // ceil(b/a)
        if (!first) out += " + ";
        out += "1 / " + to_string(c);
        first = false;
        a = a * c - b;
        b = b * c;
        long long g = std::__gcd(a < 0 ? -a : a, b);
        if (g > 1) { a /= g; b /= g; }
    }
    return out;
}

int main() {
    // README example: 4 / 13 -> 1 / 4 + 1 / 18 + 1 / 468
    cout << egyptian(4, 13) << "\n";
    return 0;
}
