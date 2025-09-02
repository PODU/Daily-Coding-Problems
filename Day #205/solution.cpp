// Day 205: Next permutation of an integer's digits.
// Standard next-permutation: find rightmost ascent, swap with next-larger suffix digit, reverse suffix.
// Time: O(d), Space: O(d) for digits.
#include <bits/stdc++.h>
using namespace std;

long long nextPermutation(long long n) {
    string s = to_string(n);
    int i = (int)s.size() - 2;
    while (i >= 0 && s[i] >= s[i + 1]) i--;
    if (i < 0) return n; // already the largest permutation
    int j = (int)s.size() - 1;
    while (s[j] <= s[i]) j--;
    swap(s[i], s[j]);
    reverse(s.begin() + i + 1, s.end());
    return stoll(s);
}

int main() {
    cout << nextPermutation(48975) << endl; // 49578
    return 0;
}
