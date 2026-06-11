// Next permutation of the digit array: find rightmost ascending pair, swap with
// next-greater suffix digit, reverse suffix. Time O(d), Space O(d).
#include <iostream>
#include <string>
#include <algorithm>
using namespace std;

long long nextPermutation(long long num) {
    string s = to_string(num);
    int n = (int)s.size();
    int i = n - 2;
    while (i >= 0 && s[i] >= s[i + 1]) i--;
    if (i < 0) return -1; // no next permutation
    int j = n - 1;
    while (s[j] <= s[i]) j--;
    swap(s[i], s[j]);
    reverse(s.begin() + i + 1, s.end());
    return stoll(s);
}

int main() {
    cout << nextPermutation(48975) << "\n";
    return 0;
}
