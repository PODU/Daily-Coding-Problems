// Day 113: Reverse word order in-place: reverse whole, then each word. O(n) time, O(1) extra.
#include <bits/stdc++.h>
using namespace std;
void reverseWords(string& s){
    reverse(s.begin(), s.end());
    int n = s.size(), start = 0;
    for(int i=0;i<=n;i++){
        if(i==n || s[i]==' '){
            reverse(s.begin()+start, s.begin()+i);
            start = i+1;
        }
    }
}
int main(){
    string s = "hello world here";
    reverseWords(s);
    cout << "\"" << s << "\"\n"; // "here world hello"
    return 0;
}
