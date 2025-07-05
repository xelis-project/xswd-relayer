# XSWD Relayer

**XSWD Relayer** is a lightweight, high-performance WebSocket tunnel designed to connect exactly two peers via a temporary secure channel.<br>

It forwards messages in both directions, with zero inspection or transformation.<br>

This makes it ideal for building **trustless communication layers**, encrypted tunnels, or peer-to-peer protocols over WebSocket.

## 🛠 How It Works

1. A client connects to `/ws` and receives a `channel UUID`.
2. Another client connects to `/ws/{uuid}` using that channel ID.
3. Once two peers are connected, all WebSocket messages from one are **proxied to the other**.
4. If either client disconnects, the channel is closed and removed.

> **Note:** The relayer is content-agnostic. You can layer encryption (e.g. AES-GCM) and authentication (e.g. HMAC, digital signatures) over the tunneled messages.

## 🔒 Why XSWD Relayer?

The [XSWD protocol](https://docs.xelis.io/features/wallet/xswd) enables **secure communication between a dApp and a XELIS wallet**. However, this becomes problematic when the wallet is running on a **separate device** — for example:

- The wallet is on your **phone**
- The dApp runs on your **desktop browser**

In this setup, direct communication is not possible. That’s where **XSWD Relayer** comes in.

It acts as a **bridge**, allowing the desktop dApp and mobile wallet to communicate safely — without trusting the relayer.

### 🧠 How it stays secure

To prevent MITM attacks or data leaks, the **JavaScript SDK**:

- Generates a **random encryption key**
- Uses it to **encrypt and authenticate** all messages exchanged with the wallet
- Embeds the **channel ID**, **encryption key**, and **relayer URL** inside a **QR Code**

> The wallet scans the QR code and connects securely to the same channel. Since all messages are end-to-end encrypted and authenticated, even if the relayer is compromised, your data remains safe.

## 📦 Usage

### Run the Server

```bash
cargo run --release
```

Also see `--help` for all the available configurations.
