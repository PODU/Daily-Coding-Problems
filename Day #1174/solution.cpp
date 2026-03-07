// Day 1174: Decide whether a string is a valid number.
// Single linear scan: optional sign, integer/fraction digits, optional exponent.
// Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool isNumber(const string& s) {
    int n = (int)s.size(), i = 0;
    if (n == 0) return false;
    auto digit = [&](int k) { return k < n && s[k] >= '0' && s[k] <= '9'; };
    if (i < n && (s[i] == '+' || s[i] == '-')) i++;
    bool before = false, after = false;
    while (digit(i)) { i++; before = true; }
    if (i < n && s[i] == '.') {
        i++;
        while (digit(i)) { i++; after = true; }
    }
    if (!before && !after) return false;
    if (i < n && (s[i] == 'e' || s[i] == 'E')) {
        i++;
        if (i < n && (s[i] == '+' || s[i] == '-')) i++;
        bool exp = false;
        while (digit(i)) { i++; exp = true; }
        if (!exp) return false;
    }
    return i == n;
}

int main() {
    vector<string> tests = {"10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"};
    for (auto& t : tests) cout << "\"" << t << "\" -> " << (isNumber(t) ? "true" : "false") << "\n";
    return 0;
}
