// Day 202: Integer palindrome check without string conversion.
// Reverse the second half of the digits and compare with the first half.
// Time: O(log10 n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

bool isPalindrome(long long x) {
    if (x < 0) return false;
    if (x != 0 && x % 10 == 0) return false; // trailing zero, not 0 itself
    long long rev = 0;
    while (x > rev) { rev = rev * 10 + x % 10; x /= 10; }
    return x == rev || x == rev / 10;
}

int main() {
    cout << boolalpha;
    cout << isPalindrome(121) << endl; // true
    cout << isPalindrome(888) << endl; // true
    cout << isPalindrome(678) << endl; // false
    return 0;
}
