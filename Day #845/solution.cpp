// Day 845: rotate a list left by k in place using the 3-reversal trick.
// reverse(0,k-1), reverse(k,n-1), reverse(0,n-1). O(n) time, O(1) extra space (~n swaps).
#include <bits/stdc++.h>
using namespace std;

void reverse(vector<int>& a, int i, int j){ while(i < j) swap(a[i++], a[j--]); }

void rotateLeft(vector<int>& a, int k){
    int n = a.size();
    if(n == 0) return;
    k %= n; if(k < 0) k += n;
    reverse(a, 0, k-1);
    reverse(a, k, n-1);
    reverse(a, 0, n-1);
}

int main(){
    vector<int> a = {1,2,3,4,5,6};
    rotateLeft(a, 2);
    cout << "[";
    for(size_t i = 0; i < a.size(); ++i){ cout << a[i]; if(i+1 < a.size()) cout << ", "; }
    cout << "]\n"; // [3, 4, 5, 6, 1, 2]
    return 0;
}
