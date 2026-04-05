// Day 1302: First missing positive integer in O(n) time, O(1) extra space.
// Cyclic placement: put value v at index v-1; first index i with a[i]!=i+1 gives answer.
#include <bits/stdc++.h>
using namespace std;

int firstMissingPositive(vector<int>& a) {
    int n = a.size();
    for (int i = 0; i < n; i++)
        while (a[i] > 0 && a[i] <= n && a[a[i] - 1] != a[i])
            swap(a[i], a[a[i] - 1]);
    for (int i = 0; i < n; i++)
        if (a[i] != i + 1) return i + 1;
    return n + 1;
}

int main() {
    vector<int> a = {3, 4, -1, 1};
    vector<int> b = {1, 2, 0};
    cout << firstMissingPositive(a) << endl; // 2
    cout << firstMissingPositive(b) << endl; // 3
    return 0;
}
