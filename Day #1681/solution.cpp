// Day 1681: Min parentheses to remove for validity. Track unmatched '(' and count
// unmatched ')'; answer = leftover open + unmatched close. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minRemovals(const string& s) {
    int open = 0, removals = 0;
    for (char c : s) {
        if (c == '(') open++;
        else if (c == ')') { if (open > 0) open--; else removals++; }
    }
    return removals + open;
}

int main() {
    cout << minRemovals("()())()") << "\n"; // 1
    cout << minRemovals(")(") << "\n";       // 2
    return 0;
}
