// Day 111: Anagram substrings via fixed sliding window of char counts. O(|S|).
#include <bits/stdc++.h>
using namespace std;
vector<int> findAnagrams(const string& s, const string& w){
    vector<int> res;
    int n = s.size(), m = w.size();
    if(m > n) return res;
    array<int,256> need{}, win{};
    for(char c : w) need[(unsigned char)c]++;
    for(int i=0;i<n;i++){
        win[(unsigned char)s[i]]++;
        if(i >= m) win[(unsigned char)s[i-m]]--;
        if(i >= m-1 && win == need) res.push_back(i-m+1);
    }
    return res;
}
int main(){
    auto r = findAnagrams("abxaba", "ab");
    for(size_t i=0;i<r.size();++i) cout << r[i] << (i+1<r.size()?", ":"\n");
    return 0;
}
