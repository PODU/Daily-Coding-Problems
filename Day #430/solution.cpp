// Day 430: Balance parentheses with the minimum number of insertions + deletions.
// One pass: keep matched pairs; unmatched ')' -> "()", leftover '(' gets a ')'. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

string balance(const string& s) {
    string res;
    int open = 0;
    for (char c : s) {
        if (c == '(') {
            open++;
            res += '(';
        } else { // ')'
            if (open > 0) {
                open--;
                res += ')';
            } else {
                res += "()";
            }
        }
    }
    while (open-- > 0) res += ')';
    return res;
}

int main() {
    cout << balance("(()") << endl;
    cout << balance("))()(") << endl;
    return 0;
}
