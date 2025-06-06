#ifndef __TEST_HPP__
#define __TEST_HPP__

#include <cstdint>
#include <array>
#include <complex>

template<uint32_t s_>
class Config{
    uint16_t    doppler_idx;
    uint16_t    range_idx;
    std::array< std::complex<float> , s_> vector;
};

#endif