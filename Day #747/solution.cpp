// B is a rotation of A iff |A|==|B| and B is a substring of A+A.
// Time: O(n) using a linear substring search (find here ~ O(n^2) worst; KMP gives O(n)).
#include <bits/stdc++.h>
using namespace std;

bool isRotation(const string& a, const string& b){
    if(a.size() != b.size()) return false;
    return (a + a).find(b) != string::npos;
}

int main(){
    cout << boolalpha;
    cout << isRotation("abcde", "cdeab") << endl; // true
    cout << isRotation("abc", "acb") << endl;     // false
    return 0;
}
