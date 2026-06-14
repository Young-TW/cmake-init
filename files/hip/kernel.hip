#include <cstdio>
#include <hip/hip_runtime.h>

__global__ void hello_kernel() {
    printf("Hello from HIP thread %d\n", threadIdx.x);
}

// Entry point declared in main.cpp; launches the GPU kernel.
void run_kernel() {
    hello_kernel<<<1, 8>>>();
    hipDeviceSynchronize();
}
