// Day 1482: Integer palindrome without converting to a string.
// Reverse the number arithmetically and compare. Time O(digits), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool isPalindrome(long long x) {
    if (x < 0) return false;
    long long rev = 0, n = x;
    while (n > 0) {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    return rev == x;
}

int main() {
    for (long long v : {121LL, 888LL, 678LL})
        cout << v << " -> " << (isPalindrome(v) ? "palindrome" : "not a palindrome") << "\n";
    // 121 -> palindrome / 888 -> palindrome / 678 -> not a palindrome
}
