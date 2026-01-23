// Day 942: Min parentheses to remove to make the string valid.
// One pass: count unmatched ')' immediately + leftover unmatched '('. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int minRemovals(const string& s) {
    int open = 0, removals = 0;
    for (char c : s) {
        if (c == '(') open++;
        else if (c == ')') {
            if (open > 0) open--;
            else removals++; // unmatched ')'
        }
    }
    return removals + open; // leftover '(' also need removal
}

int main() {
    cout << minRemovals("()())()") << "\n"; // 1
    cout << minRemovals(")(") << "\n";       // 2
    return 0;
}
