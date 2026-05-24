// Palindrome permutation: toggle chars in a set; a permutation is a palindrome
// iff at most one char has an odd count (set size <= 1). Time O(n), Space O(alphabet).
#include <bits/stdc++.h>
using namespace std;

bool canFormPalindrome(const string& s) {
    set<char> odd;
    for (char c : s) {
        if (odd.count(c)) odd.erase(c);
        else odd.insert(c);
    }
    return odd.size() <= 1;
}

int main() {
    string s = "carrace";
    cout << (canFormPalindrome(s) ? "true" : "false") << endl;
    return 0;
}
