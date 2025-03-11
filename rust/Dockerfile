FROM rust

# Install dependencies
RUN apt-get update && apt-get install -y \
    libopencv-dev \
    clang-14 \
    libclang-14-dev \
    liblept5 \
    tesseract-ocr \
    tesseract-ocr-eng \
    libtesseract-dev

RUN apt-get install -y autoconf automake libtool \
    pkg-config \
    libpng-dev \
    libjpeg62-turbo-dev \
    libtiff5-dev \
    zlib1g-dev \
    libwebpdemux2 libwebp-dev \
    libopenjp2-7-dev \
    libgif-dev \
    libarchive-dev libcurl4-openssl-dev 

ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib


CMD ["bash"]
