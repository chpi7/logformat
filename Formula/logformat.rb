class Logformat < Formula
    desc "Simply log formatting tool"
    homepage "https://github.com/chpi7/logformat"
    url "https://github.com/chpi7/logformat/archive/refs/tags/0.0.1.tar.gz"
    version "0.0.1"
    sha256 "5839f84c5c1e98b08647d2eec3706bc2fb6218712887ea0858fa2f557cc5bc34"
    license "MIT"

    depends_on "rust" => :build

    def install
        system "cargo", "build", "--release", "--bin", "logformat"
        bin.install "target/release/logformat"
    end
end