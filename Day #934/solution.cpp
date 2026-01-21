// Day 934: First recurring character = first char that has been seen before while scanning.
// Hash set of seen chars; return on first repeat. Time O(n), Space O(min(n, alphabet)).
#include <bits/stdc++.h>
using namespace std;

// Returns the recurring char, or '\0' if none.
char firstRecurring(const string& s, bool& found) {
    unordered_set<char> seen;
    for (char c : s) {
        if (seen.count(c)) { found = true; return c; }
        seen.insert(c);
    }
    found = false;
    return '\0';
}

int main() {
    for (string s : {string("acbbac"), string("abcdef")}) {
        bool found;
        char c = firstRecurring(s, found);
        if (found) cout << "\"" << c << "\"\n";
        else       cout << "null\n";
    }
    // "b"
    // null
    return 0;
}
