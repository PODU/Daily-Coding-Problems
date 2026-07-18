// Min swaps to seat couples together. Union couples in each seat-pair; answer is
// N - (#connected components). O(N * alpha(N)).
#include <bits/stdc++.h>
using namespace std;

int find(vector<int>& p, int x){ return p[x]==x ? x : p[x]=find(p, p[x]); }

int minSwaps(vector<int>& row){
    int n = row.size() / 2;            // number of couples
    vector<int> p(n);
    iota(p.begin(), p.end(), 0);
    int comps = n;
    for(int i = 0; i < n; i++){
        int a = row[2*i] / 2, b = row[2*i+1] / 2;
        int ra = find(p, a), rb = find(p, b);
        if(ra != rb){ p[ra] = rb; comps--; }
    }
    return n - comps;
}

int main(){
    vector<int> row = {0, 2, 1, 3}; // persons; couples are (0,1) and (2,3)
    cout << minSwaps(row) << "\n";  // 1
    return 0;
}
