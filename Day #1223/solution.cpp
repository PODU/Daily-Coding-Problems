// Single linear scan; depth = max paren nesting ('(' +1, ')' -1).
// O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int treeDepth(const string& s) {
    int depth = 0, cur = 0;
    for (char c : s) {
        if (c == '(') { cur++; depth = max(depth, cur); }
        else if (c == ')') cur--;
    }
    return depth;
}

int main() {
    cout << treeDepth("((((00)0)0)0)") << "\n";
    return 0;
}
