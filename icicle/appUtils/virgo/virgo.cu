#include "virgo.cuh"
#include "sumcheck.cu"

namespace virgo {
  extern "C" cudaError_t CONCAT_EXPAND(CURVE, BkSumAllCase1)(
    curve_config::scalar_t* arr1,
    curve_config::scalar_t* arr2,
    curve_config::scalar_t* output,
    int n)
  {
    return bk_sum_all_case_1<curve_config::scalar_t>(arr1, arr2, output, n);
  }

  extern "C" cudaError_t CONCAT_EXPAND(CURVE, BkSumAllCase2)(
    curve_config::scalar_t* arr,
    curve_config::scalar_t* output,
    int n)
  {
    return bk_sum_all_case_2<curve_config::scalar_t>(arr, output, n);
  }
}
