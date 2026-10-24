class PiClock < Formula
  desc "Terminal clock that displays time as multiples of pi"
  homepage "https://github.com/RUdolf1517/pi-clock"
  version "0.1.2"
  license "MIT"

  livecheck do
    url :homepage
    strategy :github_latest
  end

  on_macos do
    on_arm do
      url "https://github.com/RUdolf1517/pi-clock/releases/download/v0.1.2/pi-clock-aarch64-apple-darwin.tar.gz"
      sha256 "4fc3a2e0549800db6033d96f5fb198371b3359331f6d14d4a9cd6d231189f9e3"
    end
    on_intel do
      url "https://github.com/RUdolf1517/pi-clock/releases/download/v0.1.2/pi-clock-x86_64-apple-darwin.tar.gz"
      sha256 "cd9eb6d562f5532761bf859d621497e8f65aeedf3c16f9811fbba96cbf2067f7"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/RUdolf1517/pi-clock/releases/download/v0.1.2/pi-clock-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "2442f905d3595d45bb7e520d1ce82479761ace9d2bd89f4689ec511926ec7015"
    end
    on_arm do
      url "https://github.com/RUdolf1517/pi-clock/releases/download/v0.1.2/pi-clock-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "8e872ca8687921e9baf66b1f6fcf6f78f79c6b0196baa1fb4905c371e11df57f"
    end
  end

  def install
    bin.install "pi-clock"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/pi-clock version")
  end
end
