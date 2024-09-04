from ubuntu
run apt update
run apt -y install curl
run apt -y install libgtk-4-dev libcairo2-dev build-essential
run curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
run echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

workdir /ui