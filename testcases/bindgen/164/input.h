void test() {
    int i = 42;
    __asm    {
        mov     ecx, i
        mov     eax, ecx
    }
}