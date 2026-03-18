# CesaConn

<div align="center">
  <img src="./logo.png" alt="CesaConn Logo" width="200"/>
  
  ### Ready. Set. Connect.
  
  *CesaConn — connecting all your devices together securely.*

  ![Status](https://img.shields.io/badge/status-in%20development-yellow)
  ![License](https://img.shields.io/badge/license-GPL%203.0-blue)
  ![Language](https://img.shields.io/badge/language-Rust-orange)
  ![Coming](https://img.shields.io/badge/coming-2026%2F2027-gold)
</div>

---

## What is CesaConn?

CesaConn is a **secure, serverless, cross-platform device synchronization application** built by CesaSec.

Sync your files, clipboard, notifications, and more — across all your devices — without any central server ever seeing your data. Your data stays yours. Always.

---

## Why CesaConn?

Most sync solutions force you to trust a third party with your data. CesaConn is different:

- **No central server** — data travels directly between your devices
- **End-to-end encrypted** — nobody can read your data, not even us
- **You are in full control** — every feature can be turned on or off
- **Zero data collection** — we don't know who you are, and we don't want to
- **Every feature is off by default after updates** — you decide what to enable

---

## Security Architecture

CesaConn is built with a military-grade security stack:

| Layer | Technology | Purpose |
|---|---|---|
| Key Exchange | X25519 ECDH | Secure shared secret — never transmitted |
| Mutual Authentication | ECDH + AES-256-GCM | Verify both devices before any data transfer |
| Encryption | AES-256-GCM | Authenticated encryption with integrity |
| Key Derivation | Argon2 | Password → cryptographic key |
| Salt Generation | OS Entropy (SysRng) | Cryptographically secure randomness |
| Packet Signing | Ed25519 | Every packet signed — signatures removed after transmission |
| Memory Safety | Zeroize | Keys and secrets wiped from RAM after use |

---

### Connection Flow

```
Device A                              Device B
   │                                     │
   │──── ECDH Public Key ──────────────►│
   │◄─── ECDH Public Key ───────────────│
   │                                     │
   │  Both independently compute         │
   │  the same shared secret             │
   │  Key is NEVER transmitted           │
   │                                     │
   │  A encrypts hash(shared_secret)     │
   │  B encrypts hash(shared_secret)     │
   │                                     │
   │──── AES(hash(shared_secret)) ─────►│
   │◄─── AES(hash(shared_secret)) ──────│
   │                                     │
   │  Both decrypt and compare hashes    │
   │  If match → identity verified ✅    │
   │  If mismatch → connection rejected ❌│
   │                                     │
   │◄════ AES-256-GCM data transfer ════►│
   │      Every packet signed Ed25519    │
   │      Signatures removed after tx    │
```

This is **Mutual Authentication** — both devices verify each other before any data is exchanged. No certificate authority required. Fully peer-to-peer.

The encryption key is **mathematically derived** on both devices independently using ECDH. It never travels over the network. Even if someone intercepts every packet — they cannot decrypt the data.

This is the same cryptographic principle used by **Signal, WireGuard, and TLS 1.3**.

---

### Why this matters

| Attack | CesaConn |
|---|---|
| Man-in-the-middle | ❌ Blocked by mutual authentication |
| Packet tampering | ❌ Blocked by Ed25519 signatures |
| Replay attack | ❌ Blocked by unique nonces |
| Eavesdropping | ❌ Blocked by AES-256-GCM |
| Brute force password | ❌ Blocked by Argon2 KDF |
| Key theft | ❌ Keys wiped from RAM by Zeroize |
| Server breach | ❌ There is no server |

---

## Features

### Planned for v1.0
- [ ] File synchronization
- [ ] Clipboard sync
- [ ] Notification mirroring
- [ ] End-to-end encryption
- [ ] Mutual authentication
- [ ] Zero Trust device authorization
- [ ] Full offline / serverless operation

### Transport Support
- [ ] WiFi / LAN (TCP + UDP)
- [ ] WiFi Hotspot
- [ ] Bluetooth LE

### Platform Support
- [ ] Windows
- [ ] Linux
- [ ] Android
- [ ] macOS *(planned)*
- [ ] iOS *(under consideration)*

---

## Philosophy

> Every feature is **off by default** after updates. You decide what to enable. We don't decide for you.

CesaConn is built on the belief that software should serve the user — not the developer. No forced features. No hidden telemetry. No dark patterns.

---

## Repository Structure

```
CesaConn/
├── cesa_conn_core/          # Core cryptography library
│   ├── src/
│   │   ├── aes.rs           # AES-256-GCM encryption/decryption
│   │   ├── salt.rs          # Secure salt generation
│   │   ├── pswd_manager.rs  # Argon2 key derivation
│   │   └── lib.rs
│   └── Cargo.toml
│
└── cesa_conn_networker/     # Networking layer
    ├── src/
    │   ├── udp_networker.rs  # Device discovery (UDP broadcast)
    │   ├── tcp_networker.rs  # Data transfer (TCP)
    │   └── lib.rs
    └── Cargo.toml
```

---

## Building from Source

### Requirements
- Rust 1.75+
- Cargo

### Build

```bash
git clone https://github.com/czarekkdev/CesaConnCore
cd CesaConnCore
cargo build --release
```

### Run Tests

```bash
cargo test
```

---

## Privacy

CesaConn is designed with privacy as a core principle, not an afterthought:

- **No account required** to use the application
- **No telemetry** — we don't collect usage data
- **No analytics** — we don't track you
- **No servers** — there is nothing to breach
- **Open source core** — verify our claims yourself

---

## License

- `cesa_conn_core` — [LGPL 3.0](LICENSE)
- `cesa_conn_networker` — [GPL 3.0](LICENSE)
- CesaConn application — Proprietary (CesaSec)

---

## About CesaSec

**CesaSec** — *Where Innovation Meets Security.*

CesaConn is a product of CesaSec, an independent security-focused software company.

---

<div align="center">
  <i>Built with ❤️ and Rust 🦀</i>
</div>