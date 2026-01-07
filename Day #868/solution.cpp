// Day 868: Can any permutation of the string form a palindrome?
// Approach: count chars; palindrome possible iff at most one char has an odd count.
// Time: O(n), Space: O(alphabet).
#include <bits/stdc++.h>
using namespace std;

bool canFormPalindrome(const string& s) {
    unordered_map<char,int> cnt;
    for (char c : s) cnt[c]++;
    int odd = 0;
    for (auto& kv : cnt) odd += kv.second & 1;
    return odd <= 1;
}

int main() {
    cout << boolalpha;
    cout << canFormPalindrome("carrace") << "\n"; // true
    cout << canFormPalindrome("daily") << "\n";   // false
    return 0;
}
