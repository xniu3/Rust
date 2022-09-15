#include <stdio.h>
int Q4a(int y , int z){
    int r1 = y;
    int r2 = z;
    int r3 = r1 + r2;
    int x = r3;
    return x;
}
int main(){
    int x = Q4a(3,5);
    printf("%d",x);
}