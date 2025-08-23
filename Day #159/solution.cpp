// Day 159: First recurring character. Scan left to right tracking seen chars in
// a set; return the first already seen. Time O(n), Space O(alphabet).
#include <bits/stdc++.h>
using namespace std;

// returns the char, or '\0' if none
char firstRecurring(const string& s) {
    unordered_set<char> seen;
    for (char c : s) {
        if (seen.count(c)) return c;
        seen.insert(c);
    }
    return '\0';
}

int main() {
    char r1 = firstRecurring("acbbac");
    char r2 = firstRecurring("abcdef");
    cout << (r1 ? string(1, r1) : "null") << "\n"; // b
    cout << (r2 ? string(1, r2) : "null") << "\n"; // null
    return 0;
}
