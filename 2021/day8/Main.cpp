#include <bits/stdc++.h>

using namespace std; 

// #define VALID(n) (n == 2 || n == 4 || n == 3 || n == 7)

bool contains(string a, string b)
{
    for(int i = 0; i < b.size(); ++i)
    {
        if(a.find(b[i]) == string::npos)
            return false; 
    }
    return true; 
}

int decode(vector<string> wires, vector<string> mesg)
{
    vector<string> fixed (10); 
    int i = 0; 
    while(i < wires.size())
    {
        if(wires[i].size() == 2)
        {
            fixed[1] = wires[i]; 
            cout << "1: " << fixed[1] << endl; 
            wires.erase(wires.begin()+i); 
        }
        else if(wires[i].size() == 4)
        {
            fixed[4] = wires[i]; 
            cout << "4: " << fixed[4] << endl; 
            wires.erase(wires.begin()+i); 
        }
        else if(wires[i].size() == 3)
        {
            fixed[7] = wires[i]; 
            cout << "7: " << fixed[7] << endl; 
            wires.erase(wires.begin()+i); 
        }
        else if(wires[i].size() == 7)
        {
            fixed[8] = wires[i]; 
            cout << "8: " << fixed[8] << endl; 
            wires.erase(wires.begin()+i); 
        }
        else
            i++; 
    }
    i = 0; 
    while(i < wires.size())
    {
        if(wires[i].size() == 6 && !contains(wires[i], fixed[1]))
        {
            fixed[6] = wires[i]; 
            cout << "6: " << fixed[6] << endl; 
            wires.erase(wires.begin()+i); 
        }
        else if(wires[i].size() == 6 && contains(wires[i], fixed[4]))
        {
            fixed[9] = wires[i]; 
            cout << "9: " << fixed[9] << endl; 
            wires.erase(wires.begin()+i); 
        }
        else if(wires[i].size() == 5 && contains(wires[i], fixed[1]))
        {
            fixed[3] = wires[i]; 
            cout << "3: " << fixed[3] << endl; 
            wires.erase(wires.begin()+i); 
        }
        else 
            i++; 
    }
    i = 0; 
    while(i < wires.size())
    {
        if(wires[i].size() == 5 && contains(fixed[6], wires[i]))
        {
            fixed[5] = wires[i]; 
            cout << "5: " << fixed[5] << endl; 
            wires.erase(wires.begin()+i); 
        }
        else if(wires[i].size() == 6)
        {
            fixed[0] = wires[i]; 
            cout << "0: " << fixed[0] << endl; 
            wires.erase(wires.begin()+i); 
        }
        else 
            i++; 
    }
    fixed[2] = wires[0]; 
    cout << "2: " << fixed[2] << endl; 

    int val = 0; 
    for(int i = 0; i < mesg.size(); ++i)
    {
        for(int j = 0; j < fixed.size(); ++j)
        {
            if(contains(mesg[i], fixed[j]) && 
                contains(fixed[j], mesg[i]))
            {
                val += j; 
                val *= 10; 
                break; 
            }
        }
    }
    val /= 10; 
    return val; 
}

int main()
{
    ifstream in ("in.txt"); 
    /*
    int f = 0; 
    while(!in.eof())
    {
        string foo; 
        for(int i = 0; i < 10; ++i)
            in >> foo; 
        in >> foo; 
        string a, b, c, d; 
        in >> a >> b >> c >> d; 
        if(VALID(a.size()))
            f++; 
        if(VALID(b.size()))
            f++; 
        if(VALID(c.size()))
            f++; 
        if(VALID(d.size()))
            f++; 
    }
    cout << f << endl; 
    */
    int sum = 0; 
    while(!in.eof())
    {
        vector<string> wires (10); 
        for(int i = 0; i < 10; ++i)
            in >> wires[i]; 
        string foo; 
        in >> foo; 
        vector<string> mesg (4); 
        for(int i = 0; i < 4; ++i)
            in >> mesg[i]; 
        sum += decode(wires, mesg); 
    }
    cout << sum << endl; 
}
