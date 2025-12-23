// Next lexicographic permutation in place (classic next_permutation). O(n) time, O(1) extra space.
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
    reverse(a.begin() + i + 1, a.end());
}

void print(vector<int> a) {
    cout << "[";
    for (size_t i = 0; i < a.size(); i++) cout << a[i] << (i + 1 < a.size() ? ", " : "");
    cout << "]\n";
}

int main() {
    vector<int> a = {1, 2, 3}, b = {1, 3, 2}, c = {3, 2, 1};
    nextPermutation(a); print(a);
    nextPermutation(b); print(b);
    nextPermutation(c); print(c);
    return 0;
}
