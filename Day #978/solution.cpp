// Valid number check via single-pass finite-state parser.
// Grammar: [sign] digits [. digits] | [sign] [digits] . digits, then optional (e/E [sign] digits).
// Time: O(n); Space: O(1).
#include <bits/stdc++.h>
using namespace std;

bool isNumber(const string& s) {
    int i = 0, n = (int)s.size();
    if (n == 0) return false;
    if (s[i] == '+' || s[i] == '-') i++;

    bool digitsBefore = false, digitsAfter = false;
    while (i < n && isdigit((unsigned char)s[i])) { i++; digitsBefore = true; }
    if (i < n && s[i] == '.') {
        i++;
        while (i < n && isdigit((unsigned char)s[i])) { i++; digitsAfter = true; }
    }
    if (!digitsBefore && !digitsAfter) return false;       // mantissa needs a digit

    if (i < n && (s[i] == 'e' || s[i] == 'E')) {
        i++;
        if (i < n && (s[i] == '+' || s[i] == '-')) i++;
        bool expDigits = false;
        while (i < n && isdigit((unsigned char)s[i])) { i++; expDigits = true; }
        if (!expDigits) return false;                      // exponent needs a digit
    }
    return i == n;                                          // no trailing junk
}

int main() {
    vector<string> valid = {"10", "-10", "10.1", "-10.1", "1e5"};
    vector<string> invalid = {"a", "x 1", "a -2", "-", "", " "};
    for (auto& s : valid)   cout << "\"" << s << "\" -> " << (isNumber(s) ? "true" : "false") << "\n";
    for (auto& s : invalid) cout << "\"" << s << "\" -> " << (isNumber(s) ? "true" : "false") << "\n";
    return 0;
}
