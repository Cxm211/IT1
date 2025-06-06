#include <stdio.h>
#include <stdalign.h>
#include <stddef.h>

struct A {
    char x[3];
} __attribute__ ((aligned(64)));

struct B {
    struct A a1;
    struct A a2;
    char x;
};

// C and D are just like A and B, except that the typedef is aligned
// rather than the struct itself.
__attribute__ ((aligned(64))) typedef struct {
    char x[3];
} C;

typedef struct {
    C c1;
    C c2;
    char x;
} D;

int main() {
    printf("sizeof(struct A) = %ld\n", sizeof(struct A));
    printf("alignof(struct A) = %ld\n", alignof(struct A));
    printf("sizeof(struct B) = %ld\n", sizeof(struct B));
    printf("alignof(struct B) = %ld\n", alignof(struct B));
    printf("offset of B.x = %ld\n", offsetof(struct B, x));
    printf("\n");
    printf("sizeof(C) = %ld\n", sizeof(C));
    printf("alignof(C) = %ld\n", alignof(C));
    printf("sizeof(D) = %ld\n", sizeof(D));
    printf("alignof(D) = %ld\n", alignof(D));
    printf("offset of D.x = %ld\n", offsetof(D, x));
    return 0;
}