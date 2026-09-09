Activate:
sudo systemctl enable --now surveillance-camera

Deactivate:
sudo systemctl disable --now surveillance-camera

Test run:
export CAMERA_USERNAME=greenhouse
export CAMERA_PASSWORD=greenhouse
cargo run
