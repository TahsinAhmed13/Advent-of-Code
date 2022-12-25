#include <bits/stdc++.h>

using namespace std; 

int main()
{
    ifstream in ("in.txt"); 
    vector<int> crabs; 
    while(!in.eof())
    {
        int c; 
        char foo; 
        in >> c >> foo; 
        crabs.push_back(c); 
    }
    vector<int> fuel (2000); 
    for(int i = 0; i < fuel.size(); ++i)
    {
        for(int j = 0; j < crabs.size(); ++j)
        {
            int d = abs(i - crabs[j]); 
            fuel[i] += d * (d + 1) / 2; 
//            fuel[i] += abs(i - crabs[j]); 
        }
        cout << i << " : " << fuel[i] << endl; 
    }
    int f = fuel[0]; 
    for(int i = 1; i < fuel.size(); ++i)
    {
        f = min(f, fuel[i]); 
    } 
    cout << f << endl; 
}
