// Tree depth from nested-paren string: scan, track paren nesting, return max depth. O(N) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;
int treeDepth(const string& s) {
    int depth = 0, maxDepth = 0;
    for (char c : s) {
        if (c == '(') { depth++; maxDepth = max(maxDepth, depth); }
        else if (c == ')') depth--;
    }
    return maxDepth;
}
int main() {
    cout << treeDepth("((((00)0)0)0)") << "\n";
    return 0;
}
