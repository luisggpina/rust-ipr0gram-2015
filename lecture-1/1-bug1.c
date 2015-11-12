#include<stdlib.h>
#include<stdio.h>

void f(int n) {
    unsigned long arr[2];

    int i;
    for (i = 0 ; i < n ; i++) {
        arr[i] = 0x004005a9;
    }
}

int main(int argc, char ** argv) {
    printf("Main: %d\n", argc);
    f(atoi(argv[1]));
    return 0;
}
