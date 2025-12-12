// First N regular (5-smooth / Hamming) numbers via 3-pointer dynamic programming.
// Each number is min of next multiples by 2, 3, 5.
// Time: O(N), Space: O(N).
#include <bits/stdc++.h>
using namespace std;

vector<long long> regularNumbers(int n){
    vector<long long> h(n);
    h[0] = 1;
    int i2=0,i3=0,i5=0;
    for(int i=1;i<n;i++){
        long long n2=h[i2]*2, n3=h[i3]*3, n5=h[i5]*5;
        long long nx = min({n2,n3,n5});
        h[i]=nx;
        if(nx==n2) i2++;
        if(nx==n3) i3++;
        if(nx==n5) i5++;
    }
    return h;
}

int main(){
    auto r = regularNumbers(10);
    for(size_t i=0;i<r.size();i++){ cout << r[i]; if(i+1<r.size()) cout << " "; }
    cout << endl; // 1 2 3 4 5 6 8 9 10 12
    return 0;
}
