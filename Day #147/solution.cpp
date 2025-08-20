// Pancake sort: only primitive is reverse(lst,i,j). Repeatedly bring current max to its place. O(n^2) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

void reverse(vector<int>& a, int i, int j) {
    while (i < j) { swap(a[i], a[j]); i++; j--; }
}

void pancakeSort(vector<int>& a) {
    int n = a.size();
    for (int size = n; size > 1; size--) {
        int maxIdx = 0;
        for (int k = 1; k < size; k++) if (a[k] > a[maxIdx]) maxIdx = k;
        if (maxIdx != size - 1) {
            reverse(a, maxIdx, size - 1); // bring max to end of current window
        }
    }
}

int main() {
    vector<int> a = {3, 6, 1, 5, 2, 4};
    pancakeSort(a);
    for (size_t i = 0; i < a.size(); i++) cout << a[i] << (i + 1 < a.size() ? " " : "\n");
    return 0; // 1 2 3 4 5 6
}
