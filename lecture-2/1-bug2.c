#include<stdlib.h>
#include<stdio.h>
#include<string.h>

void print(char * msg) {
    static char * last;

    if (msg == NULL) {
        printf("%s\n", last);
    } else {
        printf("%s\n", msg);
        last = msg;
    }
}

int main(int argc, char ** argv) {
    // block 1
    {
        char user[] = "user";
        print(user);
    }

    // block 2
    {
        char pass[] = "pass";
        print(NULL);
        print(pass);
    }
}
