// Balance a parentheses string with minimum insertions+deletions (insertion-only
// greedy is provably optimal: each unmatched paren needs exactly one edit).
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

string balance(const string &s) {
    string res;
    int bal = 0;
    for (char c : s) {
        if (c == '(') {
            res += '(';
            bal++;
        } else { // c == ')'
            if (bal > 0) {
                res += ')';
                bal--;
            } else {
                // insert a '(' to match this ')'
                res += "()";
            }
        }
    }
    res.append(bal, ')'); // close any still-open '('
    return res;
}

int main() {
    cout << balance("(()") << "\n";
    cout << balance("))()(") << "\n";
    return 0;
}
