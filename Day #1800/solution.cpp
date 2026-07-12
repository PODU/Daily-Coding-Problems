// Palindrome integer check by reversing the number arithmetically (no string). Negatives are not palindromes. O(log10(n)).
#include <bits/stdc++.h>
using namespace std;

bool isPalindrome(long long x) {
    if (x < 0) return false;
    long long original = x, reversed = 0;
    while (x > 0) {
        reversed = reversed * 10 + x % 10;
        x /= 10;
    }
    return reversed == original;
}

int main() {
    cout << (isPalindrome(121) ? "true" : "false") << endl; // true
    return 0;
}
