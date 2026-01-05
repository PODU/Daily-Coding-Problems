// Day 857: Depth of tree from (lr) string representation.
// Approach: depth equals the maximum nesting level of parentheses.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int depth(const string& s) {
    int cur = 0, mx = 0;
    for (char c : s) {
        if (c == '(') { cur++; mx = max(mx, cur); }
        else if (c == ')') cur--;
    }
    return mx;
}

int main() {
    cout << depth("(00)") << endl;            // 1
    cout << depth("((00)(00))") << endl;      // 2
    cout << depth("((((00)0)0)0)") << endl;   // 4
    return 0;
}
