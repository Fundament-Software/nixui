# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2025 Fundament Research Institute <https://fundament.institute>

{ lib, pkgs, config, modulesPath, ... }:

with lib;
let
in
{
  imports = [
    "${modulesPath}/profiles/minimal.nix"
  ];

  wsl = {
    enable = true;
    wslConf.automount.root = "/mnt";
    defaultUser = "wsl-user";
    # startMenuLaunchers = true; # broken right now
    useWindowsDriver = true;

    # Enable integration with Docker Desktop (needs to be installed)
    # docker.enable = true;
  };

  fonts.packages = with pkgs; [
    pkgs.dejavu_fonts
  ];

  environment.systemPackages = with pkgs; [
    wget
    man-pages
    git
    openssh
    xsel
    home-manager
    git-remote-hg
  ];

  # boot.binfmt.emulatedSystems = [ "aarch64-linux" ];
  system.stateVersion = "24.11";

  #hardware.opengl.extraPackages = [ pkgs.mesa_drivers ];

  networking.firewall.allowedTCPPorts = [ 263 213 80 8080 ];
  networking.firewall.allowedUDPPorts = [ 263 213 80 8080 ];
  networking.firewall.trustedInterfaces = [ "tailscale0" ];
  networking.hostName = "wsl-computer"; # Define your hostname.
  networking.firewall.checkReversePath = "loose";

  services.openssh.enable = true;
  services.printing.enable = false;

  #nixpkgs.config.allowUnfree = true;
  time.timeZone = "America/Los_Angeles";

  programs.nix-ld.enable = true;

  # Enable nix flakes
  nix.distributedBuilds = true;
  nix.settings.experimental-features = [ "nix-command" "flakes" ];
  nix.settings.system-features = [ "kvm" "big-parallel" ];
  nix.settings.trusted-users = [ "root" "wsl-user" ];
}
