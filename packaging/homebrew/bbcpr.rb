class Bbcpr < Formula
  desc "Modern Rust implementation of bbcp - high-performance parallel file copy"
  homepage "https://github.com/88plug/bbcpr"
  url "https://github.com/88plug/bbcpr/archive/v0.1.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256"
  license "GPL-3.0-or-later"
  head "https://github.com/88plug/bbcpr.git", branch: "rust-rewrite"

  depends_on "rust" => :build
  depends_on "openssl@3"

  def install
    cd "rust" do
      system "cargo", "install", *std_cargo_args
    end
  end

  test do
    # Test version output
    assert_match "bbcpr version #{version}", shell_output("#{bin}/bbcpr --version")
    
    # Test basic functionality
    testfile = testpath/"test.txt"
    testfile.write "Hello from Homebrew!"
    
    system bin/"bbcpr", testfile, testpath/"test_copy.txt"
    assert_predicate testpath/"test_copy.txt", :exist?
    assert_equal testfile.read, (testpath/"test_copy.txt").read
  end
end