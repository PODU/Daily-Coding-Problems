// Valid number validator via single-pass state machine (sign/int/dot/frac/exp).
// Time: O(n) over string length, Space: O(1).
#include <iostream>
#include <string>
#include <vector>
using namespace std;

bool isValidNumber(const string& s) {
    int i = 0, n = (int)s.size();
    if (n == 0) return false;
    if (s[i] == '+' || s[i] == '-') i++;
    bool digitsBefore = false, digitsAfter = false;
    while (i < n && isdigit((unsigned char)s[i])) { i++; digitsBefore = true; }
    if (i < n && s[i] == '.') {
        i++;
        while (i < n && isdigit((unsigned char)s[i])) { i++; digitsAfter = true; }
    }
    if (!digitsBefore && !digitsAfter) return false;
    if (i < n && (s[i] == 'e' || s[i] == 'E')) {
        i++;
        if (i < n && (s[i] == '+' || s[i] == '-')) i++;
        bool expDigits = false;
        while (i < n && isdigit((unsigned char)s[i])) { i++; expDigits = true; }
        if (!expDigits) return false;
    }
    return i == n;
}

int main() {
    vector<string> tests = {"10","-10","10.1","-10.1","1e5","a","x 1","a -2","-"};
    for (const string& t : tests)
        cout << "\"" << t << "\" -> " << (isValidNumber(t) ? "True" : "False") << "\n";
    return 0;
}
