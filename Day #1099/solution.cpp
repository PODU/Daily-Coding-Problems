// Day 1099: Rotate array right by k in-place using the reversal algorithm.
// Reverse whole, reverse first k, reverse rest. Time: O(N). Space: O(1).
#include <bits/stdc++.h>
using namespace std;

void rotate(vector<int>& a, int k){
    int n = a.size();
    if (n == 0) return;
    k %= n;
    reverse(a.begin(), a.end());
    reverse(a.begin(), a.begin()+k);
    reverse(a.begin()+k, a.end());
}

int main(){
    vector<int> a = {1,2,3,4,5,6,7};
    rotate(a, 3);
    for (int i=0;i<(int)a.size();i++) cout << a[i] << (i+1<(int)a.size()?" ":"\n"); // 5 6 7 1 2 3 4
    return 0;
}
