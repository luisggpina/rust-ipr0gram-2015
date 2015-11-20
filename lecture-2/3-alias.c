#include<stdlib.h>

struct object { size_t id; };

int main() {
    struct object the_object =  { .id = 1 };

    struct object * ptr1 = &the_object;

    struct object * ptr2 = &the_object;
}
