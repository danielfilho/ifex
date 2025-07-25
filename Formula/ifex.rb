class Ifex < Formula
  desc "A modern CLI tool for managing EXIF data in JPEG, TIFF, DNG and RAW image files"
  homepage "https://github.com/danielfilho/ifex"
  version "0.100.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/danielfilho/ifex/releases/download/v0.100.0/ifex-macos-x86_64"
      sha256 "1e5ae05dd5f94e3b46f6a15cbce8edf95697b78a8cc66b5bf4919d139d7e5d08"

      def install
        bin.install "ifex-macos-x86_64" => "ifex"
      end
    end

    if Hardware::CPU.arm?
      url "https://github.com/danielfilho/ifex/releases/download/v0.100.0/ifex-macos-aarch64"
      sha256 "5e1093c779bed3554192a2f8d62b2f5c4cd014a1a3437f366dd19e383653210f"

      def install
        bin.install "ifex-macos-aarch64" => "ifex"
      end
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/danielfilho/ifex/releases/download/v0.100.0/ifex-linux-x86_64"
      sha256 "609dac9207a1036c56181c9c2295e3833f7f0318e98fb427dc9eb27970d7c5ae"

      def install
        bin.install "ifex-linux-x86_64" => "ifex"
      end
    end
  end

  test do
    system "#{bin}/ifex", "--version"
  end
end