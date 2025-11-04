// Egyptian fraction via greedy (Fibonacci/Sylvester): take ceil(b/a) each step.
// Time: O(number of terms), Space: O(1). 64-bit ints.
#include <bits/stdc++.h>
using namespace std;

int main() {
    long long a = 4, b = 13;
    string out;
    bool first = true;
    while (a != 0) {
        long long x = (b + a - 1) / a; // ceil(b/a)
        if (!first) out += " + ";
        out += "1 / " + to_string(x);
        first = false;
        a = a * x - b;
        b = b * x;
    }
    cout << out << "\n";
    return 0;
}
