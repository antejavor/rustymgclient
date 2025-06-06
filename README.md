[![Actions Status](https://github.com/memgraph/mgclient/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/memgraph/mgclient/actions)

# mgclient

mgclient is a C library interface for [Memgraph](https://www.memgraph.com)
database.

## Building and installing on Apple

To build and install mgclient from source you will need:
   - CMake version >= 3.8
   - OpenSSL version >= 1.0.2
   - Apple LLVM/clang >= 8.0.0

Once everything is in place, create a build directory inside the project
root directory and configure the build by running CMake from it:

```
mkdir build
cd build
cmake ..
```

NOTE: Dealing with OpenSSL might be a bit tricky. If OpenSSL is not available
on the system, please use, e.g., [brew](https://brew.sh/) package manager to
install OpenSSL with the following command:

```
brew install openssl@1.1
```

If `cmake` can't locate OpenSSL, please set `OPENSSL_ROOT_DIR` to a valid path.
Examples follow:

```
# M1 with brew installed
cmake -DOPENSSL_ROOT_DIR="$(brew --prefix openssl)" ..

# Using only ls command
cmake -DOPENSSL_ROOT_DIR="$(ls -rd -- /usr/local/Cellar/openssl@1.1/* | head -n 1)" ..
```

After running CMake, you should see a Makefile in the build directory. Then you
can build the project by running:

```
make
```

This will build two `mgclient` library flavours: a static library (named
`libmgclient.a`) and a shared library (named `libmgclient.dylib`).

To install the libraries and corresponding header files run:

```
make install
```

This will install to system default installation directory. If you want to
change this location, use `-DCMAKE_INSTALL_PREFIX` option when running CMake.

## Building and installing on Linux

To build and install mgclient from source you will need:
   - CMake version >= 3.8
   - OpenSSL version >= 1.0.2
   - gcc >= 8 or clang >= 8

To install minimum compile dependencies on Debian / Ubuntu:

```
apt-get install -y git cmake make gcc g++ libssl-dev
```

On RedHat / CentOS / Fedora:

```
yum install -y git cmake make gcc gcc-c++ openssl-devel
```

Once everything is in place, create a build directory inside the source
directory and configure the build by running CMake from it:

```
mkdir build
cd build
cmake ..
```

After running CMake, you should see a Makefile in the build directory. Then you
can build the project by running:

```
make
```

This will build two `mgclient` library flavours: a static library (usually
named `libmgclient.a`) and a shared library (usually named `libmgclient.so`).

To install the libraries and corresponding header files run:

```
make install
```

This will install to system default installation directory. If you want to
change this location, use `-DCMAKE_INSTALL_PREFIX` option when running CMake.

If you want to build and run tests, in the build directory run:

```
cmake -DBUILD_TESTING=ON -DBUILD_TESTING_INTEGRATION=ON ..
ctest
```

## Building and installing on Windows

To build and install mgclient from source on Windows you will need:
   - CMake version >= 3.8
   - OpenSSL version >= 1.0.2
   - MinGW: gcc >= 8 or Windows Compiler (take a look
     [here](https://blog.knatten.org/2022/08/26/microsoft-c-versions-explained/)
     to understand versioning in a bit more details):
     VS >= 17 2022, MSVC >= 14.34, C >= 19.34

### Windows Compiler

```
mkdir build
cd build
cmake ..
cmake --build .
```

Depending on where OpenSSL is installed you might need to define
`OPENSSL_ROOT_DIR`, example follows:
```
cmake -DOPENSSL_ROOT_DIR="$VCPKG_ROOT\installed\x64-windows" ..
```

To install OpenSSL [vcpkg](https://vcpkg.io/en/index.html) can be used:
```
vcpkg install openssl:x64-windows
```
or you can download and install OpenSSL from
[here](https://slproweb.com/products/Win32OpenSSL.html).

### MinGW

- Install MSYS2 from https://www.msys2.org/.
- Install MinGW toolchain with the following command:
  ```
  pacman -S --needed git base-devel mingw-w64-i686-toolchain mingw-w64-x86_64-toolchain mingw-w64-i686-cmake mingw-w64-x86_64-cmake mingw-w64-i686-openssl mingw-w64-x86_64-openssl
  ```

Once the environment is ready, please run:

```
mkdir build
cd build
cmake .. -G "MinGW Makefiles"
cmake --build . --target install
```

## Building WASM (Linux only)

Compiling `mgclient` for wasm requires the Emscripten sdk. This is automated in
the following steps:
  1. mkdir build && cd build
  2. cmake .. -DWASM=ON
  3. make

Now there should be an `mgclient.js` and an `mgclient.wasm` found in
`mgclient/build/`

## Using the library

The library provides header files located under the include folder. All library
functionality is documented in these files in Doxygen format. You can also
build HTML version of the documentation by running `doxygen` command from
project root directory.

## Examples

All the examples of the usage of the mgclient are contained in the
[examples](examples) folder, including the C++ wrapper.

An example on how to include mgclient inside a CMake project is located under
`examples/CMakeLists.txt`.
