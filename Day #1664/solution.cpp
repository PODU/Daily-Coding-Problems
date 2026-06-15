// Apply permutation: result[P[i]] = array[i]. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;
int main(){
    vector<string> arr={"a","b","c"};
    vector<int> P={2,1,0};
    int n=arr.size();
    vector<string> res(n);
    for(int i=0;i<n;++i) res[P[i]]=arr[i];
    cout<<"[";
    for(int i=0;i<n;++i){ if(i) cout<<", "; cout<<res[i]; }
    cout<<"]\n";
    return 0;
}
