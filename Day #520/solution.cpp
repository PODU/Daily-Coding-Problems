// Tree depth = max nesting level of '(' in the representation. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int treeDepth(const string& s) {
    int depth = 0, best = 0;
    for (char c : s) {
        if (c == '(') best = max(best, ++depth);
        else if (c == ')') depth--;
    }
    return best;
}

int main() {
    cout << treeDepth("((((00)0)0)0)") << "\n"; // 4
    return 0;
}
