// Day 1451: Next lexicographic permutation in place (wraps to smallest).
// Find rightmost ascent, swap with next-larger suffix element, reverse suffix.
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

void nextPermutation(vector<int>& a) {
    int n = a.size(), i = n - 2;
    while (i >= 0 && a[i] >= a[i + 1]) i--;
    if (i >= 0) {
        int j = n - 1;
        while (a[j] <= a[i]) j--;
        swap(a[i], a[j]);
    }
    reverse(a.begin() + i + 1, a.end()); // i==-1 reverses whole -> smallest
}

void show(vector<int> a) {
    nextPermutation(a);
    cout << "[";
    for (size_t i = 0; i < a.size(); i++) cout << a[i] << (i + 1 < a.size() ? "," : "");
    cout << "]\n";
}

int main() {
    show({1,2,3}); // [1,3,2]
    show({1,3,2}); // [2,1,3]
    show({3,2,1}); // [1,2,3]
    return 0;
}
