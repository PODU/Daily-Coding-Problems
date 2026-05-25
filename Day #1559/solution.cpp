// Scan parenthesis string, track open-paren depth, record maximum. Time O(n), Space O(1).
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
    string s = "((((00)0)0)0)";
    cout << treeDepth(s) << "\n";
    return 0;
}
