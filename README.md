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

To run the code, run ```cargo run``` in  RustChessEngine directory

Can run following command to generate training dataset (work in progress)
```
zstdcat *.zst |  cargo run
```

replace ```*.zst``` with whatever lichess database file you have.