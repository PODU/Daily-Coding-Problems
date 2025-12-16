// Longest palindromic substring via Manacher's algorithm.
// Transform with '#' separators, expand radii using mirror symmetry.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

string longestPalindrome(const string& s){
    if(s.empty()) return "";
    string t = "#";
    for(char c : s){ t += c; t += '#'; }
    int n = t.size();
    vector<int> p(n, 0);
    int c = 0, r = 0;
    for(int i=0;i<n;i++){
        if(i < r) p[i] = min(r - i, p[2*c - i]);
        while(i - p[i] - 1 >= 0 && i + p[i] + 1 < n && t[i-p[i]-1] == t[i+p[i]+1]) p[i]++;
        if(i + p[i] > r){ c = i; r = i + p[i]; }
    }
    int maxLen = 0, center = 0;
    for(int i=0;i<n;i++) if(p[i] > maxLen){ maxLen = p[i]; center = i; }
    int start = (center - maxLen) / 2;
    return s.substr(start, maxLen);
}

int main(){
    cout << longestPalindrome("aabcdcb") << endl; // bcdcb
    cout << longestPalindrome("bananas") << endl; // anana
    return 0;
}
