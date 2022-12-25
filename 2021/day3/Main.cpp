#include <bits/stdc++.h>

using namespace std;


int main()
{
    ifstream in ("in.txt"); 
    // vector<int> zeros (12); 
    // vector<int> ones (12); 
    vector<string> oxygen; 
    vector<string> carbon; 
    while(!in.eof())
    {
        string b; 
        in >> b; 
        oxygen.push_back(b); 
        carbon.push_back(b); 
        /*
        for(int i = 0; i < b.size(); ++i)
        {
            if(b[i] == '1')
                ones[i]++; 
            else 
                zeros[i]++; 
        }
        */
    }
    int k = 0; 
    while(oxygen.size() > 1)
    {
        int z, o; 
        z = o = 0; 
        for(int i = 0; i < oxygen.size(); ++i)
        {
            if(oxygen[i][k] == '0')
                z++; 
            else 
                o++; 
        }
        char c = o >= z ? '1' : '0'; 
        int i = 0; 
        while(i < oxygen.size())
        {
            if(oxygen[i][k] != c)
                oxygen.erase(oxygen.begin()+i); 
            else 
                i++; 
        }
        k++; 
    }
    k = 0; 
    while(carbon.size() > 1)
    {
        int z, o; 
        z = o = 0; 
        for(int i = 0; i < carbon.size(); ++i)
        {
            if(carbon[i][k] == '0')
                z++; 
            else 
                o++; 
        }
        char c = z <= o ? '0' : '1'; 
        int i = 0; 
        while(i < carbon.size())
        {
            if(carbon[i][k] != c)
                carbon.erase(carbon.begin()+i); 
            else
                i++; 
        }
        k++; 
    }
    int a, b; 
    a = b = 0; 
    for(int i = 0; i < oxygen[0].size(); ++i)
    {
        if(oxygen[0][i] == '1')
            a++; 
        if(carbon[0][i] == '1')
            b++; 
        a *= 2; 
        b *= 2; 
    }
    a /= 2; 
    b /= 2; 
    cout << (a * b) << endl; 
    /*
    int g = 0; 
    int e = 0; 
    for(int i = 0; i < zeros.size(); ++i)
    {
        if(ones[i] > zeros[i])
            g++; 
        else
            e++; 
        g *= 2; 
        e *= 2; 
    }
    g /= 2; 
    e /= 2; 
    cout << (g * e) << endl; 
    */
}
