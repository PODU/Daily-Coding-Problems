// Dutch National Flag 3-way partition (R<G<B). In-place, O(n) time, O(1) space, swaps only.
#include <bits/stdc++.h>
using namespace std;

void sortRGB(vector<char>& a) {
    int low = 0, mid = 0, high = (int)a.size() - 1;
    while (mid <= high) {
        if (a[mid] == 'R') { swap(a[low], a[mid]); low++; mid++; }
        else if (a[mid] == 'G') { mid++; }
        else { swap(a[mid], a[high]); high--; }
    }
}

int main() {
    vector<char> a = {'G', 'B', 'R', 'R', 'B', 'R', 'G'};
    sortRGB(a);
    cout << "[";
    for (size_t i = 0; i < a.size(); i++) {
        cout << "'" << a[i] << "'";
        if (i + 1 < a.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
