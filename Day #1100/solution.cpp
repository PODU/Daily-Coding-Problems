// Day 1100: Search sorted array in O(log N) using only addition/comparison
// (no *, /, or bit-shift). Binary lifting with powers of two built by doubling.
// Time: O(log N). Space: O(log N) for the power list.
#include <bits/stdc++.h>
using namespace std;

bool contains(const vector<int>& a, int x){
    int n = a.size();
    if (n == 0) return false;
    vector<int> pows;                 // 1,2,4,... <= n, built via repeated addition
    for (int p = 1; p <= n; p += p) pows.push_back(p);
    int pos = -1;                     // largest index with a[pos] <= x
    for (int i = (int)pows.size()-1; i >= 0; --i){
        int p = pows[i];
        if (pos + p < n && a[pos + p] <= x) pos += p;
    }
    return pos >= 0 && a[pos] == x;
}

int main(){
    vector<int> a = {1,3,5,7,9,11};
    cout << (contains(a, 7) ? "true" : "false") << "\n";  // true
    cout << (contains(a, 8) ? "true" : "false") << "\n";  // false
    return 0;
}
