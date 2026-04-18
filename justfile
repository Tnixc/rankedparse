remote := "aether"
remote_dir := "~/rankedparse"

# sync source to remote (excludes data/ and target/)
sync:
    rsync -avz --exclude target/ --exclude data/ --exclude .git/ . {{remote}}:{{remote_dir}}/

# sync + build on remote
build: sync
    ssh -t {{remote}} "cd {{remote_dir}} && CARGO_TERM_COLOR=always cargo build --release"

# sync + build + run on remote
run *ARGS: sync
    ssh -t {{remote}} "cd {{remote_dir}} && CARGO_TERM_COLOR=always cargo build --release && cargo run --release -- {{ARGS}}"

# run on remote without rebuilding
run-only *ARGS:
    ssh -t {{remote}} "cd {{remote_dir}} && CARGO_TERM_COLOR=always cargo run --release -- {{ARGS}}"
