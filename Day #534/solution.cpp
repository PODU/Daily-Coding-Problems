// Permutation-palindrome check: a permutation can be a palindrome iff at most one
// character has an odd frequency. Toggle membership in a set as we scan.
// Time: O(n); Space: O(alphabet).
#include <bits/stdc++.h>
using namespace std;

bool canPermutePalindrome(const string& s) {
    unordered_set<char> odd;
    for (char ch : s) {
        if (odd.count(ch)) odd.erase(ch);
        else odd.insert(ch);
    }
    return odd.size() <= 1;
}

int main() {
    cout << (canPermutePalindrome("carrace") ? "true" : "false") << "\n";
    cout << (canPermutePalindrome("daily") ? "true" : "false") << "\n";
    return 0;
}
