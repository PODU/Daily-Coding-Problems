// Prefix-sum hashmap: store sum->index; if sum-K seen, slice between. O(n) time, O(n) space.
#include <iostream>
#include <vector>
#include <unordered_map>
using namespace std;
int main(){
    vector<int> a={1,2,3,4,5};int K=9;
    unordered_map<long long,int> seen;seen[0]=-1;long long s=0;int lo=-1,hi=-1;
    for(int i=0;i<(int)a.size();++i){s+=a[i];
        if(seen.count(s-K)){lo=seen[s-K]+1;hi=i;break;}
        if(!seen.count(s))seen[s]=i;}
    cout<<"[";for(int i=lo;i<=hi;++i)cout<<a[i]<<(i<hi?", ":"");cout<<"]\n";return 0;}
