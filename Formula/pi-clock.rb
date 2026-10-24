class PiClock < Formula
  desc "Terminal clock that displays time as multiples of pi"
  homepage "https://github.com/RUdolf1517/pi-clock"
  version "0.1.3"
  license "MIT"

  livecheck do
    url :homepage
    strategy :github_latest
  end

  on_macos do
    on_arm do
      url "https://github.com/RUdolf1517/pi-clock/releases/download/v0.1.3/pi-clock-aarch64-apple-darwin.tar.gz"
      sha256 "04dd766c3f327270bf0224329e64fe80fa7d5e5dc6f62626d8fa3c5d1d302f33"
    end
    on_intel do
      url "https://github.com/RUdolf1517/pi-clock/releases/download/v0.1.3/pi-clock-x86_64-apple-darwin.tar.gz"
      sha256 "da1e935abd1c1e9bac3aa29c33cc9a213e5992f65871002c6116ed04001274bb"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/RUdolf1517/pi-clock/releases/download/v0.1.3/pi-clock-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "4eb0756840d26bbd311e8772359924a0e61380770f67c426fe31ee0df4e6284d"
    end
    on_arm do
      url "https://github.com/RUdolf1517/pi-clock/releases/download/v0.1.3/pi-clock-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "57b08ac761e270372c2c9773f36fd79d00891ab3bf05c9418daadcac06cd382a"
    end
  end

  def install
    bin.install "pi-clock"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/pi-clock version")
  end
end
