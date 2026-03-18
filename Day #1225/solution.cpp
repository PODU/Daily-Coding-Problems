// Min parens to remove for validity: single pass counting unmatched.
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minRemovals(const string& s) {
    int open = 0, removals = 0;
    for (char c : s) {
        if (c == '(') open++;
        else if (c == ')') {
            if (open > 0) open--;
            else removals++;
        }
    }
    return removals + open;
}

int main() {
    cout << minRemovals("()())()") << "\n";
    cout << minRemovals(")(") << "\n";
    return 0;
}
