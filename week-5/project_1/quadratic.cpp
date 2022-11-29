#include <iostream>
#include <math.h>
using namespace std;

int main ()
{
    int a = 0;
    int b = 0;
    int c = 0;
    
    cout<<"Enter your values"<<endl;
    cout<<"a"<<endl;
    cin>>a;
    cout<<"b"<<endl;
    cin>>b;
    cout<<"c"<<endl;
    cin>>c;
    float f1 = powf(b,2);
    float formula = sqrtf(f1 - 4*a*c); 
    float p1 = -b + formula;
    float p2 = -b - formula;
    float X1 = p1/2*a;
    float X2 = p2/2*a;
    cout<<"X1 ="<<X1<<endl;
    cout<<"X2 ="<<X2<<endl;
    
    
    return 0;

}