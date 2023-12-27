```
If training, run the following to get stockfish
sudo apt install stockfish
```

# Pytorch/tch setup
Enabling CUDA on WSL: https://docs.nvidia.com/cuda/wsl-user-guide/index.html

Run the following commands to set up conda venv in top level directory of git repo:

```
conda env create -f requirements.yml --name <env-name>
conda activate <env-name>
```

You will probably need to add the following to .bashrc (or run this command each time):

export LD_LIBRARY_PATH=/path/to/libtorch/lib:$LD_LIBRARY_PATH

You can find /path/to/libtorch/lib by running python3 in the terminal and run:

```
>> import torch
>> torch.__path__[0]
```

add /lib to the end of the result of torch.__path__[0]. Use this path to replace /path/to/libtorch/lib.


To run the code, run the following in  Rust ChessEngine directory

```
LIBTORCH_USE_PYTORCH=1   cargo run
```
