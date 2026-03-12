# zsh-chat: Product Definition (v2.0)

## The Vision: "Lo-Fi High-Tech"
A decentralized social protocol for the terminal that prioritizes cryptographic sovereignty, mathematical discovery, and a low-fidelity aesthetic.

## Core Pillars

### 1. The Fibonacci Feed (Discovery)
Instead of a black-box algorithm, the feed is governed by the Golden Ratio ($\phi \approx 1.618$).
- **1st Order (Direct - 61.80%):** Posts from your direct follows.
- **2nd Order (Mutuals - 23.61%):** Prioritized by "Path Density."
- **3rd Order (Extended - 9.02%):** Friends of friends of friends.
- **4th Order (Distant - 3.44%):**
- **5th Order (Trace - 1.32%):**
- **6th Order (Fringe - 0.50%):**
- **The "Anti-Bubble":** Remaining percentage reserved for global "New Entrants."

### 2. Identity & Trust
- **Keypair Sovereignty:** Ed25519 keys generated locally. No central accounts.
- **Proof of Work (PoW):** Every message requires a Hashcash-style PoW. This acts as a "Spam Tax."
- **Web of Trust:** Identity verification is done by checking signatures against a local "Known Keys" database.

### 3. Lo-Fi Aesthetics (The "Render Engine")
- **Static Emoji Mapping:** Standard Unicode emojis are detected and replaced visually by a "4-bit ANSI Art" version baked into the binary. The raw message preserves the Unicode for copy-pasting.
- **Dynamic Image Downsampling:** Images pasted/uploaded are dynamically downsampled to 16 ANSI colors (4-bit) or 4 ANSI colors (2-bit) using Floyd-Steinberg dithering.
- **Soft Moderation:** Low-res rendering removes the impact of shock-content while preserving the "vibe."

### 4. Decentralized Relays
- **Dumb Relays:** WebSocket servers that store and forward signed JSON events.
- **Incentivization:** Running a relay unlocks "Extended Capability" (e.g., local art packs, special TUI badges).
- **Embedded Relay:** The `zsh-chat` binary includes the relay code (`zsh-chat serve`).
