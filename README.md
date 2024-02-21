# MKL-RS

Rust bindings for MKL.

## Install MKL

Install MKL on your system from the [site](https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl-download.html).

For example on Fedora, run:

```sh
sudo dnf install \
    intel-oneapi-mkl \
    intel-oneapi-mkl-devel \
    intel-oneapi-compiler-shared-runtime-2024.0 \
    intel-oneapi-openmp-common-2024.0 \
    intel-oneapi-openmp-2024.0 \
    libgomp \
    libgomp-devel \
    libomp
```


Ensure pkg-config can find your MKL isntallation. Installing on Debian-based or
RHEL-based distributions will configure pkg-config correctly by default.

You can check that `mkl` can be found by running:

```sh
$ pkg-config --cflags mkl-static-ilp64-gomp
-DMKL_ILP64 -m64 -I/opt/intel/oneapi/mkl/2024.0/lib/pkgconfig/../../include
```

