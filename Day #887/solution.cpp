// Egyptian fraction via greedy: take ceil(b/a) each step. Time O(terms), Space O(1).
#include <bits/stdc++.h>
using namespace std;

string egyptian(long long a, long long b) {
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
    return out;
}

int main() {
    cout << egyptian(4, 13) << endl;
    return 0;
}
