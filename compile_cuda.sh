#!/bin/bash
set -e

# Убедимся, что мы в нужной директории
cd "$(dirname "$0")"

# Пути
CUDA_PATH=/usr
NVCC=/usr/bin/nvcc
SRC_DIR=src/cuda-kernels
OUT_DIR=build
LIB_NAME=libcuda_vanity.a

mkdir -p $OUT_DIR

# Компиляция основного файла (он включает остальные через #include)
echo "Compiling vanity.cu..."
$NVCC -ccbin clang-14 -allow-unsupported-compiler \
      -rdc=true -DENDIAN_NEUTRAL -DLTC_NO_ASM \
      -Xcompiler "-isystem /usr/include/c++/13 -isystem /usr/include/x86_64-linux-gnu/c++/13 -isystem /usr/include" \
      -gencode arch=compute_50,code=sm_50 \
      -gencode arch=compute_61,code=sm_61 \
      -gencode arch=compute_70,code=sm_70 \
      -gencode arch=compute_80,code=sm_80 \
      -gencode arch=compute_86,code=sm_86 \
      -gencode arch=compute_89,code=sm_89 \
      -gencode arch=compute_90,code=sm_90 \
      -I. -I$SRC_DIR \
      -c $SRC_DIR/vanity.cu -o $OUT_DIR/vanity.o

OBJ_FILES="$OUT_DIR/vanity.o"

# Device link
echo "Linking objects (device link)..."
$NVCC -ccbin clang-14 -dlink -o $OUT_DIR/dlink.o $OBJ_FILES \
      -gencode arch=compute_50,code=sm_50 \
      -gencode arch=compute_61,code=sm_61 \
      -gencode arch=compute_70,code=sm_70 \
      -gencode arch=compute_80,code=sm_80 \
      -gencode arch=compute_86,code=sm_86 \
      -gencode arch=compute_89,code=sm_89 \
      -gencode arch=compute_90,code=sm_90

# Создание статической библиотеки
echo "Creating static library $LIB_NAME..."
ar rcs $LIB_NAME $OBJ_FILES $OUT_DIR/dlink.o

echo "Done!"
