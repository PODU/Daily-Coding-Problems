// Day 114: Reverse words but keep delimiters fixed. Collect words, reverse list,
// re-emit walking original structure. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;
string reverseKeepDelims(const string& s, const set<char>& delim){
    vector<string> words;
    string cur;
    for(char c : s){
        if(delim.count(c)){ if(!cur.empty()){ words.push_back(cur); cur.clear(); } }
        else cur += c;
    }
    if(!cur.empty()) words.push_back(cur);
    reverse(words.begin(), words.end());
    string out; int wi = 0; size_t i = 0;
    while(i < s.size()){
        if(delim.count(s[i])){ out += s[i]; i++; }
        else { out += words[wi++]; while(i < s.size() && !delim.count(s[i])) i++; }
    }
    return out;
}
int main(){
    set<char> d = {'/', ':'};
    cout << reverseKeepDelims("hello/world:here", d) << "\n";  // here/world:hello
    cout << reverseKeepDelims("hello/world:here/", d) << "\n"; // here/world:hello/
    cout << reverseKeepDelims("hello//world:here", d) << "\n"; // here//world:hello
    return 0;
}
