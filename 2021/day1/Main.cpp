#include <bits/stdc++.h>

using namespace std; 

int main()
{
    ifstream in ("in.txt"); 
    /*
    int a, b; 
    in >> a; 
    int t = 0; 
    while(!in.eof())
    {
        in >> b; 
        if(b > a)
            t++;
        a = b; 
    }
    cout << t << endl; 
    */
    int a, b, c, d; 
    in >> a >> b >> c; 
    int t = 0; 
    while(!in.eof())
    {
        in >> d; 
        if(a < d)
            t++; 
        a = b; 
        b = c; 
        c = d; 
    }
    cout << t << endl; 
}
