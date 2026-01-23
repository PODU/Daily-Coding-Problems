// Day 944: Next greater permutation of an integer's digits (in-place on digit array).
// Find pivot, swap with next larger from the right, reverse suffix. Time O(d), Space O(d).
#include <bits/stdc++.h>
using namespace std;

// Returns -1 if no greater permutation exists.
long long nextPermutation(long long num) {
    string d = to_string(num);
    int n = (int)d.size();
    int i = n - 2;
    while (i >= 0 && d[i] >= d[i + 1]) i--;
    if (i < 0) return -1; // already the largest arrangement
    int j = n - 1;
    while (d[j] <= d[i]) j--;
    swap(d[i], d[j]);
    reverse(d.begin() + i + 1, d.end());
    return stoll(d);
}

int main() {
    cout << nextPermutation(48975) << "\n"; // 49578
    return 0;
}
