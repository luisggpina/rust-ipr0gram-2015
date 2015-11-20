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
    char * last;

    // block 1
    {
        char user[] = "user";
        // print(user);
        {
            char * msg = user;
            printf("%s\n", msg);
            last = msg;
        }
    }

    // block 2
    {
        char pass[] = "pass";
        // print(NULL);
        {
            printf("%s\n", last);
        }
        // print(pass);
        {
            char * msg = pass;
            printf("%s\n", msg);
            last = msg;
        }
    }
}
