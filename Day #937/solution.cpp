// Day 937: Valid parenthesis string with '*' (= '(' , ')' or empty).
// Greedy: track [lo,hi] range of possible open counts. Valid iff lo can reach 0. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool isValid(const string& s) {
    int lo = 0, hi = 0; // range of unmatched '(' counts
    for (char c : s) {
        if (c == '(') { lo++; hi++; }
        else if (c == ')') { lo--; hi--; }
        else { lo--; hi++; } // '*'
        if (hi < 0) return false;
        if (lo < 0) lo = 0;
    }
    return lo == 0;
}

int main() {
    vector<string> in = {"(()*", "(*)", ")*("};
    vector<string> bal, notbal;
    for (auto& s : in) (isValid(s) ? bal : notbal).push_back(s);

    auto join = [](const vector<string>& v) {
        string r;
        for (size_t i = 0; i < v.size(); ++i) r += (i ? " and " : "") + v[i];
        return r;
    };
    cout << join(bal) << " are balanced. " << join(notbal) << " is not balanced.\n";
    // (()* and (*) are balanced. )*( is not balanced.
    return 0;
}
