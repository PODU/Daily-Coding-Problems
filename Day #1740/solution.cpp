// Approach: deterministic single linear scan validating sign/digits/dot/exponent.
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool isNumber(const string& s) {
    int n = s.size(), i = 0;
    if (n == 0) return false;
    if (s[i] == '+' || s[i] == '-') i++;
    bool digits = false, dot = false;
    while (i < n && (isdigit((unsigned char)s[i]) || s[i] == '.')) {
        if (s[i] == '.') {
            if (dot) return false;
            dot = true;
        } else digits = true;
        i++;
    }
    if (!digits) return false;
    if (i < n && (s[i] == 'e' || s[i] == 'E')) {
        i++;
        if (i < n && (s[i] == '+' || s[i] == '-')) i++;
        bool expDigits = false;
        while (i < n && isdigit((unsigned char)s[i])) { expDigits = true; i++; }
        if (!expDigits) return false;
    }
    return i == n;
}

int main() {
    vector<string> tests = {"10","-10","10.1","-10.1","1e5","a","x 1","a -2","-"};
    for (auto& t : tests) cout << (isNumber(t) ? "true" : "false") << "\n";
    return 0;
}
