// Balanced parens with '*' wildcard: track range [lo,hi] of possible open counts. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool isBalanced(const string& s) {
    int lo = 0, hi = 0;
    for (char c : s) {
        if (c == '(') { lo++; hi++; }
        else if (c == ')') { lo--; hi--; }
        else { lo--; hi++; } // '*' as ')', '(' or empty
        if (hi < 0) return false;
        if (lo < 0) lo = 0;
    }
    return lo == 0;
}

int main() {
    string a = "(()*", b = "(*)", c = ")*(";
    cout << (isBalanced(a) ? "(()*" : "") << " and " << (isBalanced(b) ? "(*)" : "")
         << " are balanced. " << (!isBalanced(c) ? ")*(" : "") << " is not balanced." << endl;
    return 0;
}
