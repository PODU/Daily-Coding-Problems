// Day 123: Validate whether a string is a number (int/real/scientific).
// Single linear scan state machine. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool isNumber(const string& s) {
    int i = 0, n = s.size();
    if (n == 0) return false;
    if (s[i] == '+' || s[i] == '-') i++;
    int digits = 0, dots = 0;
    while (i < n && (isdigit((unsigned char)s[i]) || s[i] == '.')) {
        if (s[i] == '.') dots++;
        else digits++;
        i++;
    }
    if (dots > 1 || digits == 0) return false;
    if (i < n && (s[i] == 'e' || s[i] == 'E')) {
        i++;
        if (i < n && (s[i] == '+' || s[i] == '-')) i++;
        int expd = 0;
        while (i < n && isdigit((unsigned char)s[i])) { expd++; i++; }
        if (expd == 0) return false;
    }
    return i == n;
}

int main() {
    vector<string> tests = {"10", "-10", "10.1", "-10.1", "1e5", "a", "x 1", "a -2", "-"};
    for (auto& t : tests)
        cout << "\"" << t << "\" -> " << (isNumber(t) ? "true" : "false") << endl;
    return 0;
}
