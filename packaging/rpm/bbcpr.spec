Name:           bbcpr
Version:        0.1.0
Release:        1%{?dist}
Summary:        Modern parallel file copy utility

License:        GPL-3.0-or-later
URL:            https://github.com/88plug/bbcpr
Source0:        https://github.com/88plug/bbcpr/archive/v%{version}/%{name}-%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  openssl-devel
BuildRequires:  pkgconfig

%description
bbcpr is a modern Rust implementation of bbcp, providing high-performance
parallel file transfers with features including:

- Multi-stream parallel transfers for maximum speed
- SSH and TCP connection support
- Real-time progress reporting
- Multiple checksum algorithms (MD5, CRC32, Adler32)
- Cross-platform support
- Memory-safe implementation

%prep
%autosetup -n bbcpr-%{version}

%build
cd rust
cargo build --release

%install
cd rust
install -Dm755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
cd ..
install -Dm644 LICENSE %{buildroot}%{_datadir}/licenses/%{name}/LICENSE
install -Dm644 README.md %{buildroot}%{_docdir}/%{name}/README.md

%check
cd rust
cargo test --release

%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}

%changelog
* Sat Jul 12 2025 Andrew Mello <andrew@88plug.com> - 0.1.0-1
- Initial package release
- Modern Rust implementation of bbcp
- Multi-stream parallel file transfers
- SSH and TCP connection support