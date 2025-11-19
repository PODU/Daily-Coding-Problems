// Minimum parentheses to remove to make string valid: single pass counting unmatched
// open/close. Answer = removals (unmatched ')') + leftover open. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minRemove(const string& s) {
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
    cout << minRemove("()())()") << endl; // 1
    cout << minRemove(")(") << endl;       // 2
    return 0;
}
