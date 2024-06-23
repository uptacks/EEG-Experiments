# EEG-Experiments
Some experiments utilizing the OpenBCI EEG Biosensing apparatus

## Introduction

Words words words words words

## Setup

This repo requires you to setup the brainflow[https://brainflow.readthedocs.io/en/stable/BuildBrainFlow.html] library to operate. This repo expects the correctly compiled brainflow folder to be put into the root of the EEG-Experiments folder. It is recommended to follow the brainflow installation instructions for your operating system. Below is a step by step guide for MacOS.

### MacOS

1. Clone the brainflow repo
```bash
git clone https://github.com/brainflow-dev/brainflow.git
```

2. Install cmake 

3. Build the brainflow C++ library 
```bash
cd brainflow
mkdir build
cd build
cmake ..
make
```

4. Build the brainflow rust library
```bash
cd ..
cd rust_package
cargo build --features generate_binding
```

5. Copy the rust_packages/brainflow folder into the root of the EEG-Experiments folder

