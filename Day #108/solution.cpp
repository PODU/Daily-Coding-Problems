// Day 108: B is a rotation of A iff |A|==|B| and B is a substring of A+A. O(n).
#include <bits/stdc++.h>
using namespace std;
bool isRotation(const string& a, const string& b){
    if(a.size() != b.size()) return false;
    return (a + a).find(b) != string::npos;
}
int main(){
    cout << (isRotation("abcde","cdeab") ? "true" : "false") << "\n";
    cout << (isRotation("abc","acb") ? "true" : "false") << "\n";
    return 0;
}
