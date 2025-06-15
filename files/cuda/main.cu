#include <cuda_runtime.h>

#include <stdio.h>

__global__ void helloFromGPU() {
    printf("Hello from GPU thread %d\n", threadIdx.x);
}

int main() {
    // Launch the kernel with 10 threads
    helloFromGPU<<<1, 10>>>();

    // Wait for GPU to finish before accessing results
    cudaDeviceSynchronize();

    return 0;
}