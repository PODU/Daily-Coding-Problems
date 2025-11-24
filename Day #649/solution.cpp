// First recurring character: single pass with a hash set, return first char already seen.
// Time O(n), Space O(k).
#include <bits/stdc++.h>
using namespace std;

// Returns the recurring char, or '\0' if none.
char firstRecurring(const string& s) {
    unordered_set<char> seen;
    for (char c : s) {
        if (seen.count(c)) return c;
        seen.insert(c);
    }
    return '\0';
}

void run(const string& s) {
    char r = firstRecurring(s);
    if (r == '\0') cout << "null" << "\n";
    else cout << r << "\n";
}

int main() {
    run("acbbac");
    run("abcdef");
    return 0;
}
