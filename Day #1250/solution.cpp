// Palindrome permutation: count char parities; valid iff <=1 char has odd count. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

bool canPermutePalindrome(const string& s) {
    unordered_map<char,int> cnt;
    for (char c : s) cnt[c]++;
    int odd = 0;
    for (auto& p : cnt) if (p.second & 1) odd++;
    return odd <= 1;
}

int main() {
    cout << (canPermutePalindrome("carrace") ? "true" : "false") << "\n";
    cout << (canPermutePalindrome("daily") ? "true" : "false") << "\n";
    return 0;
}
