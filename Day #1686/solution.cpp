// Selection sort using only reverse(lst,i,j). For each i, find min in [i..n-1],
// reverse [i..m] to bring it to front. Time O(n^2), Space O(1).
#include <iostream>
#include <vector>
using namespace std;

void reverseRange(vector<int>& lst, int i, int j) {
    while (i < j) { swap(lst[i], lst[j]); i++; j--; }
}

void sortWithReverse(vector<int>& lst) {
    int n = (int)lst.size();
    for (int i = 0; i < n; i++) {
        int m = i;
        for (int k = i + 1; k < n; k++) if (lst[k] < lst[m]) m = k;
        reverseRange(lst, i, m);
    }
}

int main() {
    vector<int> lst = {3, 1, 2, 5, 4};
    sortWithReverse(lst);
    for (size_t i = 0; i < lst.size(); i++) {
        cout << lst[i];
        if (i + 1 < lst.size()) cout << ' ';
    }
    cout << '\n';
    return 0;
}
