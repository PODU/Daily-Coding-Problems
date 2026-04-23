// Pancake sort: for each shrinking prefix find its max, flip it to the front,
// then flip it into its final position. Only uses reverse(lst,i,j).
// Time O(n^2) comparisons, O(n) flips, Space O(1).
#include <bits/stdc++.h>
using namespace std;

void reverse(vector<int>& a, int i, int j) {
    while (i < j) { swap(a[i], a[j]); i++; j--; }
}

void pancakeSort(vector<int>& a) {
    for (int n = a.size(); n > 1; n--) {
        int mi = 0;
        for (int i = 1; i < n; i++) if (a[i] > a[mi]) mi = i;
        if (mi != n - 1) {
            reverse(a, 0, mi);      // max to front
            reverse(a, 0, n - 1);   // front to end of unsorted region
        }
    }
}

int main() {
    vector<int> a = {3, 1, 4, 1, 5, 9, 2, 6};
    pancakeSort(a);
    for (size_t i = 0; i < a.size(); i++) cout << a[i] << (i + 1 < a.size() ? " " : "\n");
    return 0;
}
