#include <bits/stdc++.h>

using namespace std; 
typedef long long ll; 

int main()
{
    ifstream in ("in.txt"); 
    ll x, y, z; 
    x = y = z = 0; 
    while(!in.eof())
    {
        string d; 
        ll n; 
        in >> d >> n; 
        if(!d.compare("forward"))
        {
            x += n; 
            y += n * z; 
        }
        else if(!d.compare("down"))
        {
            z += n; 
        }
        else if(!d.compare("up"))
        {
            z -= n; 
        }
    }
    cout << (x * y) << endl; 
}
