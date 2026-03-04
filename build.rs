fn main() {
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", project_dir);
    println!("cargo:rustc-link-lib=static=cuda_vanity");
    
    // CUDA Runtime и необходимые библиотеки
    println!("cargo:rustc-link-lib=cudart");
    println!("cargo:rustc-link-lib=cuda");
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
    
    // Поскольку мы использовали clang для хост-линковки в CUDA, 
    // нам могут понадобиться стандартные библиотеки C++
    println!("cargo:rustc-link-lib=stdc++");
    
    // Добавим зависимости для pthread, так как CUDA их использует
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=dl");
}
