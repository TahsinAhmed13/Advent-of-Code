#include <bits/stdc++.h>

using namespace std; 
typedef long long ll; 

int w, x, y, z; 
unordered_map<char, int*> var = {
    {'w', &w},
    {'x', &x}, 
    {'y', &y}, 
    {'z', &z}
}; 

void inp(int * a, int b){ *a = b; }
void add(int * a, int b){ *a += b; }
void mul(int * a, int b){ *a *= b; }
void div(int * a, int b){ *a = trunc((double) *a / b); }
void mod(int * a, int b){ *a %= b; }
void eql(int * a, int b){ *a = *a == b; }
unordered_map<string, decltype(&inp)> func = {
    {"inp", &inp}, 
    {"add", &add}, 
    {"mul", &mul}, 
    {"div", &div}, 
    {"mod", &mod}, 
    {"eql", &eql}
}; 

bool isDigit(char c)
{
    return '0' <= c && c <= '9'; 
}

void execute(vector<string> instr, int input)
{
    for(string line : instr)
    {
        char f[3]; 
        int * a; 
        int b; 
        char v, u; 
        if(!line.substr(0, 3).compare("inp"))
        {
            sscanf(line.c_str(), "%s %c", f, &v); 
            b = input; 
        }
        else if(isDigit(line[6]) || line[6] == '-')
        {
            sscanf(line.c_str(), "%s %c %d", f, &v, &b); 
        }
        else
        {
            sscanf(line.c_str(), "%s %c %c", f, &v, &u); 
            b = *(var[u]); 
        }
        a = var[v]; 
        func[string(f)](a, b); 
        printf("w: %d, x: %d, y: %d, z: %d\n", w, x, y, z); 
    }
}

vector<vector<string>> split(vector<string> input)
{
    vector<vector<string>> instrs; 
    for(string line : input)
    {
        if(!line.substr(0, 3).compare("inp"))
            instrs.push_back(vector<string>());
        instrs[instrs.size()-1].push_back(line); 
    }
    return instrs; 
}

ll part1(vector<string> input)
{
    int lw, lx, ly, lz; 
    w = x = y = z = 0; 
    lw = lx = ly = lz = 0; 
    ll model = 0; 
    for(vector<string> instr : split(input))
    {
        ll d = 9; 
        while(d > 0)
        {
            w = lw; x = lx; y = ly; z = lz; 
            execute(instr, d); 
            if(!z)  break; 
            d--; 
        }
        model += d; 
        model *= 10; 
        lw = w; lx = x; ly = y; lz = z; 
    }
    return model / 10; 
}

int main()
{
    ifstream in ("in.txt"); 
    vector<string> input; 
    while(!in.eof())
    {
        string line; 
        getline(in, line); 
        input.push_back(line); 
    }
    cout << "Part 1: " << part1(input) << endl; 
}
