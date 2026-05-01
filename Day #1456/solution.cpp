// Day 1456: First recurring character in a string.
// Approach: scan left-to-right, track seen chars in a hash set; first char
// already seen is the answer. Time O(n), Space O(1) (fixed alphabet).
#include <bits/stdc++.h>
using namespace std;

// Returns the first recurring character, or '\0' if none.
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
        if (found) cout << '"' << c << '"' << "\n";
        else cout << "null" << "\n";
    }
    return 0;
}
