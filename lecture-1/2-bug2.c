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
    char * user = malloc(5 * sizeof(char));
    memcpy(user, "user", 5);

    print(user);

    free(user);

    char * pass = malloc(5 * sizeof(char));
    memcpy(pass, "pass", 5);

    print(NULL);
    print(pass);
}
