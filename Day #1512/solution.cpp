// First recurring character: scan L->R, track seen set; first char already seen wins.
// O(n) time, O(alphabet) space.
#include <bits/stdc++.h>
using namespace std;

// Returns the first recurring char, or '\0' if none.
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
    bool found;
    char r = firstRecurring("acbbac", found);
    if (found) cout << r << "\n";
    else cout << "null" << "\n";
    return 0;
}
