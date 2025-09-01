// Day 199: Balance parentheses with minimum insertions/deletions.
// Greedy: each unmatched paren costs exactly 1 op; inserting its partner is always optimal.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

string balance(const string& s) {
    string res;
    int open = 0;
    for (char c : s) {
        if (c == '(') { res += '('; open++; }
        else {
            if (open > 0) { res += ')'; open--; }
            else res += "()"; // unmatched ')': insert a '(' before it
        }
    }
    while (open-- > 0) res += ')'; // close remaining opens
    return res;
}

int main() {
    cout << balance("(()") << endl;   // (())
    cout << balance("))()(") << endl; // ()()()()
    return 0;
}
