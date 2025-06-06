struct {
    char *decl_4_0[0][4]; // fails
} struct_2_0;

// struct {
//     char decl_4_0[0][4]; // succeeds
// } struct_2_0;

// struct {
//     char *decl_4_0[1][4]; // succeeds
// } struct_2_0;