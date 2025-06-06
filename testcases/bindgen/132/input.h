#include <stddef.h>
#include <stdint.h>

typedef uint32_t mbedtls_mpi_uint;

/**
 * \brief          MPI structure
 */
typedef struct
{
    int s;              /*!<  integer sign      */
    size_t n;           /*!<  total # of limbs  */
    mbedtls_mpi_uint *p;          /*!<  pointer to limbs  */
}
mbedtls_mpi;