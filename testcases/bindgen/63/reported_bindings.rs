/* Registers for entry into PLT on x86-64.  */                                        
# if __GNUC_PREREQ (4,0)                                                              
typedef float La_x86_64_xmm __attribute__ ((__vector_size__ (16)));                   
typedef float La_x86_64_ymm                                                           
    __attribute__ ((__vector_size__ (32), __aligned__ (16)));                         
typedef double La_x86_64_zmm                                                          
    __attribute__ ((__vector_size__ (64), __aligned__ (16)));                         
# else                                                                                
typedef float La_x86_64_xmm __attribute__ ((__mode__ (__V4SF__)));                    
# endif

typedef union                                                                         
{                                                                                     
# if __GNUC_PREREQ (4,0)                                                              
  La_x86_64_ymm ymm[2];                                                               
  La_x86_64_zmm zmm[1];                                                               
# endif                                                                               
  La_x86_64_xmm xmm[4];                                                               
} La_x86_64_vector __attribute__ ((__aligned__ (16)));