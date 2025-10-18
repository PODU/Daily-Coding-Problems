// Day 450: Balanced parentheses with '*' wildcards via greedy low/high open
// count. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool isBalanced(const string& s) {
    int low = 0, high = 0; // range of possible open-paren counts
    for (char c : s) {
        if (c == '(') { low++; high++; }
        else if (c == ')') { low--; high--; }
        else { low--; high++; } // '*' could be ')' , '(' , or empty
        if (high < 0) return false;       // too many ')'
        if (low < 0) low = 0;             // treat surplus '*' as not ')'
    }
    return low == 0;
}

int main() {
    string s = "(()*";
    cout << (isBalanced(s) ? "balanced" : "not balanced") << "\n"; // balanced
    return 0;
}
