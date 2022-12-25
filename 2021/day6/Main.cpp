#include <bits/stdc++.h>

using namespace std; 
typedef long long ll; 

int main()
{
    ifstream in ("in.txt"); 
    ll fish = 0;  
    vector<ll> freq (9); 
    while(!in.eof())
    {
        int a; 
        char foo; 
        in >> a >> foo; 
        freq[a]++; 
        fish++; 
    }
    for(int i = 0; i < 256; ++i)
    {
        ll tmp = freq[0]; 
        for(int i = 0; i < freq.size()-1; ++i)
        {
            freq[i] = freq[i+1];  
        }
        freq[6] += tmp; 
        freq[8] = tmp; 
        fish += tmp; 
    }
    cout << fish << endl; 
}
