// Day 1110: Detect a Pythagorean triplet a^2+b^2=c^2 in an array.
// Square all, sort; for each largest c use two-pointer scan. Time: O(N^2). Space: O(N).
#include <bits/stdc++.h>
using namespace std;

bool hasTriplet(vector<int> a){
    for (int& x : a) x = x * x;
    sort(a.begin(), a.end());
    int n = a.size();
    for (int c = n-1; c >= 2; c--){
        int l = 0, r = c-1;
        while (l < r){
            int s = a[l] + a[r];
            if (s == a[c]) return true;
            if (s < a[c]) l++; else r--;
        }
    }
    return false;
}

int main(){
    cout << (hasTriplet({3,1,4,6,5}) ? "true" : "false") << "\n"; // true (3,4,5)
    cout << (hasTriplet({10,4,6,12,5}) ? "true" : "false") << "\n"; // false
    return 0;
}
