#include <bits/stdc++.h>

using namespace std; 
typedef long long ll; 

int part1(vector<string> input)
{
    set<vector<int>> on; 
    for(string line : input)
    {
        char state[3]; 
        int xmin, xmax; 
        int ymin, ymax; 
        int zmin, zmax; 
        sscanf(line.c_str(), "%s x=%d..%d,y=%d..%d,z=%d..%d", 
                state, 
                &xmin, &xmax, 
                &ymin, &ymax, 
                &zmin, &zmax); 
        xmin = max(-50, xmin); 
        xmax = min(50, xmax); 
        ymin = max(-50, ymin); 
        ymax = min(50, ymax); 
        zmin = max(-50, zmin); 
        zmax = min(50, zmax); 
        for(int x = xmin; x <= xmax; ++x)
        {
            for(int y = ymin; y <= ymax; ++y)
            {
                for(int z = zmin; z <= zmax; ++z)
                {
                    if(!strcmp(state, "on"))
                        on.insert(vector<int>{x, y, z}); 
                    else 
                        on.erase(vector<int>{x, y, z}); 
                }
            }
        }
    }
    return on.size(); 
}

struct Bounds
{
    ll xmin, xmax; 
    ll ymin, ymax; 
    ll zmin, zmax; 
}; 

bool isValid(struct Bounds b)
{
    return b.xmin <= b.xmax && 
        b.ymin <= b.ymax && 
        b.zmin <= b.zmax; 
}

ll getArea(struct Bounds b)
{
    ll w = b.xmax - b.xmin + 1; 
    ll h = b.ymax - b.ymin + 1; 
    ll l = b.zmax - b.zmin + 1; 
    return w * h * l; 
}

struct Bounds getOverlap(struct Bounds a, struct Bounds b)
{
    struct Bounds overlap; 
    overlap.xmin = max(a.xmin, b.xmin); 
    overlap.xmax = min(a.xmax, b.xmax); 
    overlap.ymin = max(a.ymin, b.ymin); 
    overlap.ymax = min(a.ymax, b.ymax); 
    overlap.zmin = max(a.zmin, b.zmin); 
    overlap.zmax = min(a.zmax, b.zmax); 
    return overlap; 
}

ll part2(vector<string> input)
{
    vector<vector<struct Bounds>> cuboids(input.size()); 
    for(int i = 0; i < input.size(); ++i)
    {
        char state[3]; 
        struct Bounds a; 
        sscanf(input[i].c_str(), "%s x=%lld..%lld,y=%lld..%lld,z=%lld..%lld", 
                state, 
                &a.xmin, &a.xmax, 
                &a.ymin, &a.ymax, 
                &a.zmin, &a.zmax); 
        for(int j = i-1; j >= 0; --j)
        {
            for(struct Bounds b : cuboids[j])
            {
                struct Bounds c = getOverlap(a, b); 
                if(isValid(c))
                    cuboids[j+1].push_back(c); 
            }
        }
        if(!strcmp(state, "on"))
            cuboids[0].push_back(a); 
    }
    ll total = 0; 
    ll sign = 1; 
    for(int i = 0; i < cuboids.size(); ++i)
    {
        for(int j = 0; j < cuboids[i].size(); ++j)
            total += sign * getArea(cuboids[i][j]); 
        sign *= -1; 
    }
    return total; 
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
    cout << "Part 2: " << part2(input) << endl; 
}
