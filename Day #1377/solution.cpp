// Balanced parens with '*' wildcard: greedy track [lo,hi] of possible open counts.
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool isValid(const string& s) {
    int lo = 0, hi = 0;
    for (char c : s) {
        if (c == '(') { lo++; hi++; }
        else if (c == ')') { lo--; hi--; }
        else { lo--; hi++; }            // '*' as ')', '(', or empty
        if (hi < 0) return false;
        if (lo < 0) lo = 0;
    }
    return lo == 0;
}

int main() {
    vector<string> tests = {"(()*", "(*)", ")*("};
    vector<string> bal, notBal;
    for (auto& s : tests) (isValid(s) ? bal : notBal).push_back(s);
    auto join = [](vector<string>& v) {
        string r;
        for (size_t i = 0; i < v.size(); i++) r += (i ? " and " : "") + v[i];
        return r;
    };
    cout << join(bal) << " are balanced. " << join(notBal) << " is not balanced.\n";
    return 0;
}
