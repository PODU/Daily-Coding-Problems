// Day 157: A permutation is a palindrome iff at most one char has odd count.
// Track parity via a hash set of odd-count chars. Time O(n), Space O(alphabet).
#include <bits/stdc++.h>
using namespace std;

bool canFormPalindrome(const string& s) {
    unordered_set<char> odd;
    for (char c : s) {
        if (odd.count(c)) odd.erase(c);
        else odd.insert(c);
    }
    return odd.size() <= 1;
}

int main() {
    cout << (canFormPalindrome("carrace") ? "true" : "false") << "\n";
    cout << (canFormPalindrome("daily") ? "true" : "false") << "\n";
    return 0;
}
