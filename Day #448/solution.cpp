// Day 448: Dutch National Flag sort of R/G/B. O(n) time, O(1) space, in-place
// with three pointers (low=R boundary, high=B boundary, mid=scanner).
#include <bits/stdc++.h>
using namespace std;

void sortRGB(vector<char>& a) {
    int low = 0, mid = 0, high = a.size() - 1;
    while (mid <= high) {
        if (a[mid] == 'R') swap(a[low++], a[mid++]);
        else if (a[mid] == 'G') mid++;
        else swap(a[mid], a[high--]); // 'B'
    }
}

int main() {
    vector<char> a = {'G','B','R','R','B','R','G'};
    sortRGB(a);
    cout << "[";
    for (size_t i = 0; i < a.size(); ++i) cout << "'" << a[i] << "'" << (i + 1 < a.size() ? ", " : "");
    cout << "]\n"; // ['R', 'R', 'R', 'G', 'G', 'B', 'B']
    return 0;
}
