// Balance parentheses with min insertions/deletions via insertion-based scan.
// Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

string balance(const string& s) {
    string result;
    int open = 0;
    for (char c : s) {
        if (c == '(') { result.push_back('('); open++; }
        else { // ')'
            if (open == 0) { result.push_back('('); result.push_back(')'); }
            else { result.push_back(')'); open--; }
        }
    }
    result.append(open, ')');
    return result;
}

int main() {
    cout << balance("(()") << "\n";
    cout << balance("))()(") << "\n";
    return 0;
}
