cd Stockfish/src
make -j build
mv stockfish ../..
python3 -m pip install --user virtualenv
python3 -m venv env
source env/bin/activate
