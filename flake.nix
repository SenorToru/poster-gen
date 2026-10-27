# =============================================================================
# flake.nix
# 目的：用 devShell 固定 poster-gen 的开发工具链（Rust/WASM 构建链 + node / pnpm）。
#       取代原来的「rustup + cargo install trunk worker-build」，换机器不用手工装。
# 用法：交互式 shell 进入目录时由 direnv 自动加载（.envrc）。
#       非交互上下文（Claude Code 的 Bash 工具、VSCode task、systemd）里 direnv 不生效，
#       必须写成 `direnv exec . <命令>`。
# 结构照抄 shukuba/flake.nix，两个项目的 Rust 环境保持一致。
# =============================================================================
{
  description = "poster-gen development environment";

  inputs = {
    # 与 nixos-config 同一个 channel（nixos-26.05），不用 unstable。
    # 不强求和 nixos-config 锁同一个 rev：版本分叉不是正确性问题。
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    # nixos-26.05 的 rustc 停在 1.95.0，要 1.99.0 只能从 unstable 取。
    # 只用它的 rustc / cargo / clippy / rustfmt，其余工具仍走上面的稳定 channel。
    nixpkgs-rust.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, nixpkgs-rust, ... }:
    let
      systems = [ "x86_64-linux" ];
      forAllSystems =
        f:
        nixpkgs.lib.genAttrs systems (
          system: f nixpkgs.legacyPackages.${system} nixpkgs-rust.legacyPackages.${system}
        );
    in
    {
      devShells = forAllSystems (pkgs: rustPkgs: {
        default = pkgs.mkShell {
          buildInputs = [
            # rust-toolchain.toml 要 stable + wasm32-unknown-unknown + rustfmt + clippy。
            # 用 nixpkgs 的 rustc / cargo 时 cargo 不读那个文件，所以这几样在这里显式给。
            # wasm32-unknown-unknown 的 std 由 nixpkgs 的 rustc 自带。
            rustPkgs.rustc
            rustPkgs.cargo
            rustPkgs.clippy
            rustPkgs.rustfmt
            # rustc 的 wasm32 target 默认用 rust-lld，而 nixpkgs 的 rustc 不带它；
            # 不加 lld 的话编译能过、链接会报 `linker lld not found`。
            pkgs.lld
            # 前端打包（crates/app）。trunk 自己下载匹配 Cargo.lock 的 wasm-bindgen，
            # 所以这里不放 wasm-bindgen-cli。
            pkgs.trunk
            # Worker 打包（crates/worker）。同样自己下载匹配版本的 wasm-bindgen。
            pkgs.worker-build
            # wrangler 和 tailwindcss 是 devDependency，由 pnpm 管理，
            # 用 `pnpm exec` 或 `pnpm run` 调用，避免同时存在两个版本。
            pkgs.nodejs_24
            pkgs.pnpm_11
          ];

          # workerd（wrangler dev 用）静态链接 BoringSSL，CA 查找路径编译时写死，
          # NixOS 上一张 CA 都找不到，Worker 往上游发的 fetch 会全部 TLS 失败。
          # 与 shukuba、craft-crm 相同，显式指定 cacert 的证书包。
          # 必须写在 shellHook 里：写成 mkShell 属性的话，nix print-dev-env
          # （nix-direnv 用的就是它）会把 SSL_CERT_FILE 丢掉。
          shellHook = ''
            export SSL_CERT_FILE="${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
            echo "poster-gen devShell: rustc $(rustc --version | cut -d' ' -f2), trunk $(trunk --version | cut -d' ' -f2), worker-build $(worker-build --version 2>&1 | head -1), node $(node -v), pnpm $(pnpm -v)" >&2
          '';
        };
      });
    };
}
