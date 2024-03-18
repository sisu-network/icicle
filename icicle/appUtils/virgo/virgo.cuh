#pragma once
#ifndef __VIRGO_H
#define __VIRGO_H

#include <cstdint>
#include <iostream>

#include "curves/curve_config.cuh"
#include "utils/error_handler.cuh"
#include "utils/device_context.cuh"

#include "utils/utils.h"

namespace virgo {
  template <typename S>
  cudaError_t bk_sum_all_case_1(S* arr1, S* arr2, S output, int n);

  template <typename S>
  cudaError_t bk_sum_all_case_2(S* arr, S output, int n);

  template <typename S>
  cudaError_t bk_produce_case_1(S* arr1, S* arr2, S output, int n);

  template <typename S>
  cudaError_t bk_produce_case_2(S* arr1, S* arr2, S output, int n);
}

#endif