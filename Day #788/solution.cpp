// Integer palindrome check by arithmetic reversal (no string). O(log n) time, O(1) space. Negatives -> false.
#include <bits/stdc++.h>
using namespace std;

bool isPalindrome(long long n) {
    if (n < 0) return false;
    long long rev = 0, x = n;
    while (x > 0) {
        rev = rev * 10 + x % 10;
        x /= 10;
    }
    return rev == n;
}

int main() {
    cout << (isPalindrome(121) ? "true" : "false") << "\n";
    return 0;
}
