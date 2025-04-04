# 🧹 pingy

```text
       _
 _ __ (_)_ __   __ _ _   _
| '_ \| | '_ \ / _` | | | |
| |_) | | | | | (_| | |_| |
| .__/|_|_| |_|\__, |\__, |
|_|            |___/ |___/
```

⚡ Pingy: A fast, concurrent Rust-based ping tool with real-time updates & visual charts! 📊🚀

## ✨ Features

- ⚡ **Concurrent Ping**: Ping multiple addresses simultaneously for faster results.
- 📊 **Visual Latency Display**: Graphical representation of latency for easy analysis.
- ⏱️ **Real-Time Metrics**: Displays max, min, avg latency, packet loss rate, and more in real-time.
- 🌍 **IPv4 & IPv6 Support**: Works seamlessly with both IP versions.
- 🔄 **Multi-IP Under One Address**: Supports concurrent pinging of multiple IPs under a single domain.

## 🚀 Installation

To install **pingy**, simply clone the repository and follow the instructions below:

```bash
git clone git@github.com:trinhminhtriet/pingy.git
cd pingy

cargo build --release

rm -rf /usr/local/bin/pingy && ln -s ${PWD}/target/release/pingy /usr/local/bin/pingy && which pingy && pingy --version
```

Running the below command will globally install the `pingy` binary.

```bash
cargo install pingy
```

Optionally, you can add `~/.cargo/bin` to your PATH if it's not already there

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## 💡 Usage

```bash
pingy www.baidu.com www.google.com www.apple.com www.sina.com -c 20 -i 2

pingy --help

⚡ Pingy: A fast, concurrent Rust-based ping tool with real-time updates & visual charts!

Usage: pingy [OPTIONS] <TARGET>...

Arguments:
  <TARGET>...  target IP address or hostname to ping

Options:
  -c, --count <COUNT>        Number of pings to send [default: 65535]
  -i, --interval <INTERVAL>  Interval in seconds between pings [default: 0]
  -6, --force_ipv6           Force using IPv6
  -m, --multiple <MULTIPLE>  Specify the maximum number of target addresses, Only works on one target address [default: 0]
  -v, --view-type <VIEW_TYPE>  view mode graph/table/point [default: graph]
  -h, --help                 Print help
  -V, --version              Print version
```

## 🙏 Acknowledgements

Thanks to these people for their feedback and suggestions for Pingy!
[TBD]

## 🤝 How to contribute

We welcome contributions!

- Fork this repository;
- Create a branch with your feature: `git checkout -b my-feature`;
- Commit your changes: `git commit -m "feat: my new feature"`;
- Push to your branch: `git push origin my-feature`.

Once your pull request has been merged, you can delete your branch.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
