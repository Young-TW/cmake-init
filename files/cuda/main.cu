#include <cuda_runtime.h>

#include <stdio.h>

__global__ void hello_kernel() {
    printf("Hello from CUDA kernel!\n");
}

int main() {
    // Launch the kernel with 10 threads
    hello_kernel<<<1, 1>>>();

    // Wait for GPU to finish before accessing results
    cudaDeviceSynchronize();

    return 0;
}