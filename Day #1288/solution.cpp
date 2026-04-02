// Day 1288: Balance parentheses with minimum insertions+deletions (insertion-only is minimal).
// Single scan: pair each ')' with an open, else insert '(' before it; close leftover opens. O(n).
#include <bits/stdc++.h>
using namespace std;

string balance(const string& s) {
    string res;
    int open = 0;
    for (char ch : s) {
        if (ch == '(') { res += '('; ++open; }
        else {
            if (open > 0) { res += ')'; --open; }
            else { res += "()"; } // insert matching '(' before the unmatched ')'
        }
    }
    res += string(open, ')');
    return res;
}

int main() {
    cout << balance("(()") << "\n";    // (())
    cout << balance("))()(") << "\n";  // ()()()()
    return 0;
}
