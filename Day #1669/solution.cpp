// Two-pointer merge from both ends of sorted array; larger abs square goes to back. O(n) time, O(n) space.
#include <iostream>
#include <vector>
using namespace std;
int main(){
    vector<int> a={-9,-2,0,2,3};
    int n=a.size();vector<long long> r(n);int l=0,h=n-1;
    for(int p=n-1;p>=0;--p){long long lo=(long long)a[l]*a[l],hi=(long long)a[h]*a[h];
        if(lo>hi){r[p]=lo;++l;}else{r[p]=hi;--h;}}
    cout<<"[";for(int i=0;i<n;++i)cout<<r[i]<<(i+1<n?", ":"");cout<<"]\n";return 0;}
