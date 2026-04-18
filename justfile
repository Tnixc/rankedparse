remote := "aether"
remote_dir := "~/rankedparse"

# sync source to remote (excludes data/ and target/)
sync:
    rsync -avz --exclude target/ --exclude data/ --exclude .git/ . {{remote}}:{{remote_dir}}/

# sync + build on remote
build: sync
    ssh -t {{remote}} "cd {{remote_dir}} && CARGO_TERM_COLOR=always cargo build --release"

# sync + build + run on remote, then sync output back
run *ARGS: sync
    ssh -t {{remote}} "cd {{remote_dir}} && CARGO_TERM_COLOR=always cargo build --release && time cargo run --release -- {{ARGS}}"
    rsync -avz {{remote}}:{{remote_dir}}/output/ ./output/

# run on remote without rebuilding, then sync output back
run-only *ARGS:
    ssh -t {{remote}} "cd {{remote_dir}} && CARGO_TERM_COLOR=always cargo run --release -- {{ARGS}}"
    rsync -avz {{remote}}:{{remote_dir}}/output/ ./output/
