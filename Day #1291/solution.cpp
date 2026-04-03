// Day 1291: Next permutation of an integer's digits.
// Standard next-permutation: scan from right, swap, reverse suffix. O(D) time, O(D) space.
#include <bits/stdc++.h>
using namespace std;

string nextPermutation(string s) {
    int n = s.size(), i = n - 2;
    while (i >= 0 && s[i] >= s[i + 1]) i--;
    if (i < 0) return s; // already largest; no next permutation
    int j = n - 1;
    while (s[j] <= s[i]) j--;
    swap(s[i], s[j]);
    reverse(s.begin() + i + 1, s.end());
    return s;
}

int main() {
    string n = "48975";
    cout << nextPermutation(n) << endl; // 49578
    return 0;
}
