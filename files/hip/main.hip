#include <hip/hip_runtime.h>

#include <stdio.h>

__global__ void hello_kernel() {
    printf("Hello from HIP kernel!\n");
}

int main() {
    // Launch the kernel with one block and one thread
    hello_kernel<<<1, 1>>>();

    // Wait for the GPU to finish
    hipDeviceSynchronize();

    return 0;
}