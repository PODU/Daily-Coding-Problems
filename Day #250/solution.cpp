// Cryptarithmetic solver: backtracking over distinct letters with leading-zero pruning.
// Time: O(10!/(10-k)!) over k<=10 distinct letters; Space: O(k).
#include <bits/stdc++.h>
using namespace std;

int main() {
    string w1 = "SEND", w2 = "MORE", w3 = "MONEY";
    // First-appearance order across SEND, MORE, MONEY.
    vector<char> order;
    set<char> seen;
    auto add = [&](const string& w){ for(char c: w) if(!seen.count(c)){ seen.insert(c); order.push_back(c);} };
    add(w1); add(w2); add(w3);

    set<char> leading = {w1[0], w2[0], w3[0]};
    int k = order.size();
    vector<int> assign(k, -1);
    vector<bool> used(10, false);

    function<bool(int)> dfs = [&](int idx)->bool{
        if(idx == k){
            map<char,long long> val;
            for(int i=0;i<k;i++) val[order[i]] = assign[i];
            auto num = [&](const string& w){ long long n=0; for(char c: w) n=n*10+val[c]; return n; };
            return num(w1) + num(w2) == num(w3);
        }
        for(int d=0; d<10; d++){
            if(used[d]) continue;
            if(d==0 && leading.count(order[idx])) continue;
            used[d]=true; assign[idx]=d;
            if(dfs(idx+1)) return true;
            used[d]=false; assign[idx]=-1;
        }
        return false;
    };

    dfs(0);
    cout << "{";
    for(int i=0;i<k;i++){
        cout << "'" << order[i] << "': " << assign[i];
        if(i+1<k) cout << ", ";
    }
    cout << "}" << endl;
    return 0;
}
