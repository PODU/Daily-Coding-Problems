// Simplified Elo rating system. Expected score Ea=1/(1+10^((Rb-Ra)/400)),
// update Ra'=Ra+K*(Sa-Ea), K=32. recordGame transfers points winner<-loser. O(1) per game.
#include <bits/stdc++.h>
using namespace std;

struct EloSystem {
    static constexpr double K = 32.0;
    map<string,double> rating;
    void addPlayer(const string& name){ if(!rating.count(name)) rating[name]=1200.0; }
    double expected(double ra,double rb){ return 1.0/(1.0+pow(10.0,(rb-ra)/400.0)); }
    void recordGame(const string& winner,const string& loser){
        addPlayer(winner); addPlayer(loser);
        double ra=rating[winner], rb=rating[loser];
        double ea=expected(ra,rb), eb=expected(rb,ra);
        rating[winner]=ra+K*(1.0-ea);
        rating[loser]=rb+K*(0.0-eb);
    }
    int get(const string& name){ return (int)llround(rating[name]); }
};

int main(){
    EloSystem elo;
    elo.addPlayer("A"); elo.rating["A"]=1200.0;
    elo.addPlayer("B"); elo.rating["B"]=2000.0;
    cout<<"Before: A="<<elo.get("A")<<", B="<<elo.get("B")<<"\n";
    elo.recordGame("A","B"); // lower-rated A beats higher-rated B
    cout<<"After A(1200) beats B(2000): A="<<elo.get("A")<<", B="<<elo.get("B")<<"\n";
    return 0;
}
