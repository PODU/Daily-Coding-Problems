// Custom JSON serializer for null/number/string/list/dict (recursive).
// Time: O(total size), Space: O(depth).
#include <bits/stdc++.h>
using namespace std;

struct JV {
    enum T { NUL, INT, STR, LST, DCT } t = NUL;
    long long i = 0;
    string s;
    vector<JV> lst;
    vector<pair<string,JV>> dct;
    static JV Null(){ JV v; v.t=NUL; return v; }
    static JV Int(long long x){ JV v; v.t=INT; v.i=x; return v; }
    static JV Str(string x){ JV v; v.t=STR; v.s=move(x); return v; }
    static JV List(vector<JV> x){ JV v; v.t=LST; v.lst=move(x); return v; }
    static JV Dict(vector<pair<string,JV>> x){ JV v; v.t=DCT; v.dct=move(x); return v; }
};

string esc(const string& s){
    string o = "\"";
    for(char c : s){ if(c=='"'||c=='\\') o+='\\'; o+=c; }
    o += "\"";
    return o;
}

string encode(const JV& v){
    switch(v.t){
        case JV::NUL: return "null";
        case JV::INT: return to_string(v.i);
        case JV::STR: return esc(v.s);
        case JV::LST: {
            string o="["; for(size_t i=0;i<v.lst.size();i++){ if(i) o+=", "; o+=encode(v.lst[i]); } return o+"]";
        }
        case JV::DCT: {
            string o="{"; for(size_t i=0;i<v.dct.size();i++){ if(i) o+=", "; o+=esc(v.dct[i].first)+": "+encode(v.dct[i].second); } return o+"}";
        }
    }
    return "null";
}

int main(){
    JV data = JV::List({ JV::Null(), JV::Int(123),
                         JV::List({ JV::Str("a"), JV::Str("b") }),
                         JV::Dict({ {"c", JV::Str("d")} }) });
    cout << "'" << encode(data) << "'\n";
    return 0;
}
