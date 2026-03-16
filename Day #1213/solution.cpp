// Day 1213: Stable marriage via Gale-Shapley (men propose).
// Each free man proposes down his list; women keep their best suitor. Time O(N^2), Space O(N^2).
#include <bits/stdc++.h>
using namespace std;

map<string,string> stableMatch(map<string,vector<string>> guys, map<string,vector<string>> gals) {
    map<string,map<string,int>> rank; // woman -> man -> rank
    for (map<string,vector<string>>::iterator it = gals.begin(); it != gals.end(); ++it)
        for (int i = 0; i < (int)it->second.size(); i++) rank[it->first][it->second[i]] = i;
    map<string,int> nextIdx;          // man -> index of next woman to propose
    map<string,string> engaged;       // woman -> man
    deque<string> freeMen;
    for (map<string,vector<string>>::iterator it = guys.begin(); it != guys.end(); ++it) {
        freeMen.push_back(it->first); nextIdx[it->first] = 0;
    }
    while (!freeMen.empty()) {
        string m = freeMen.front(); freeMen.pop_front();
        string w = guys[m][nextIdx[m]++];
        if (!engaged.count(w)) engaged[w] = m;
        else {
            string cur = engaged[w];
            if (rank[w][m] < rank[w][cur]) { engaged[w] = m; freeMen.push_back(cur); }
            else freeMen.push_back(m);
        }
    }
    return engaged; // woman -> man
}

int main() {
    map<string,vector<string>> guys;
    guys["andrew"]  = {"caroline","abigail","betty"};
    guys["bill"]    = {"caroline","betty","abigail"};
    guys["chester"] = {"betty","caroline","abigail"};
    map<string,vector<string>> gals;
    gals["abigail"]  = {"andrew","bill","chester"};
    gals["betty"]    = {"bill","andrew","chester"};
    gals["caroline"] = {"bill","chester","andrew"};
    map<string,string> m = stableMatch(guys, gals);
    map<string,string> byMan;
    for (map<string,string>::iterator it = m.begin(); it != m.end(); ++it) byMan[it->second] = it->first;
    for (map<string,string>::iterator it = byMan.begin(); it != byMan.end(); ++it)
        cout << it->first << " - " << it->second << "\n";
    // andrew - abigail / bill - caroline / chester - betty
    return 0;
}
