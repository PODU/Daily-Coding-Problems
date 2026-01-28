// Cryptarithmetic 3-word solver (word1 + word2 = word3) via backtracking.
// Time: O(10!/(10-k)!) for k unique letters; Space: O(k).
#include <bits/stdc++.h>
using namespace std;

string A, B, C;
vector<char> letters;
set<char> leading;
map<char,int> assign;
bool usedDigit[10] = {false};

long long val(const string& w){ long long v=0; for(char ch: w) v=v*10+assign[ch]; return v; }

bool dfs(int i){
    if(i==(int)letters.size()) return val(A)+val(B)==val(C);
    char c = letters[i];
    for(int d=0; d<10; d++){
        if(usedDigit[d]) continue;
        if(d==0 && leading.count(c)) continue;
        usedDigit[d]=true; assign[c]=d;
        if(dfs(i+1)) return true;
        usedDigit[d]=false;
    }
    return false;
}

int main(){
    A="SEND"; B="MORE"; C="MONEY";
    set<char> seen;
    for(string* w : {&A,&B,&C})
        for(char ch : *w)
            if(!seen.count(ch)){ seen.insert(ch); letters.push_back(ch); }
    leading = {A[0], B[0], C[0]};
    dfs(0);
    string order = "SENDMORY";
    cout << "{";
    for(size_t i=0;i<order.size();i++){
        cout << "'" << order[i] << "': " << assign[order[i]];
        if(i+1<order.size()) cout << ", ";
    }
    cout << "}" << endl;
    return 0;
}
