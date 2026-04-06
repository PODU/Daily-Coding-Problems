// Egyptian fraction via Fibonacci/Sylvester greedy: repeatedly subtract the
// largest unit fraction 1/ceil(b/a). Time O(#terms), Space O(1).
#include <bits/stdc++.h>
using namespace std;

string egyptian(long long a, long long b) {
    string res;
    bool first = true;
    while (a != 0) {
        long long x = (b + a - 1) / a; // ceil(b/a)
        if (!first) res += " + ";
        res += "1 / " + to_string(x);
        a = a * x - b;
        b = b * x;
        first = false;
    }
    return res;
}

int main() {
    cout << egyptian(4, 13) << "\n"; // 1 / 4 + 1 / 18 + 1 / 468
    return 0;
}
