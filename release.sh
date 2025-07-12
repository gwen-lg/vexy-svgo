# Only update the workspace.package.version, not dependency versions or rust-version
sed -i.bak "/^\[workspace.package\]/,/^\[/ s/version = \"[^\"]*\"/version = \"$VERSION\"/" Cargo.toml
