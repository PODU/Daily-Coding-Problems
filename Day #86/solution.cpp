// Day 86: Min parentheses to remove for validity. Track unmatched '(' and ')'.
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minRemoval(const string& s) {
    int open = 0, removals = 0;
    for (char c : s) {
        if (c == '(') open++;
        else if (c == ')') {
            if (open > 0) open--;
            else removals++;       // unmatched ')'
        }
    }
    return removals + open;        // leftover unmatched '('
}

int main() {
    cout << minRemoval("()())()") << "\n"; // 1
    cout << minRemoval(")(") << "\n";       // 2
    return 0;
}
