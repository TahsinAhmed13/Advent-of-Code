#include <bits/stdc++.h>

using namespace std; 

int xmin, xmax, ymin, ymax; 

bool inBounds(int x, int y)
{
    return xmin <= x && x <= xmax &&
        ymin <= y && y <= ymax; 
}

bool sim(int vx, int vy)
{
    int x, y; 
    x = y = 0; 
    int hmax = 0; 
    while(x <= xmax && y >= ymin)
    {
        x += vx; 
        if(vx > 0)
            vx--; 
        y += vy; 
        vy--; 
        hmax = max(hmax, y); 
        if(inBounds(x, y))
//            return hmax; 
            return true; 
    }
    return false; 
}

int main()
{
    ifstream in ("in.txt"); 
    string s; 
    getline(in, s); 
    sscanf(s.c_str(), "target area: x=%d..%d, y=%d..%d", 
            &xmin, &xmax, &ymin, &ymax);  
//    int hmax = 0; 
    int p = 0; 
    for(int vx = 1; vx <= 200; ++vx)
    {
        for(int vy = -200; vy <= 200; ++vy)
        {
//            hmax = max(hmax, sim(vx, vy)); 
            p += sim(vx, vy); 
        }
    }
//    cout << hmax << endl; 
    cout << p << endl; 
}
