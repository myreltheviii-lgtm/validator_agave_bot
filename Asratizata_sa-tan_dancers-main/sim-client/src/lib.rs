// sim-client/src/lib.rs
//
// The shared IPC contract between the Agave validator process and the
// sim-server process.
//
// This crate carries zero Solana SDK dependency by design. Both the validator
// and the sim-server binary depend on this crate. The validator uses SimClient
// to send simulation requests and receive results. The sim-server uses the
// wire types to deserialize incoming requests and serialize responses back.
//
// The deliberate use of [u8; 32] for every public key — instead of any SDK
// Pubkey type — is what makes zero-SDK possible. A Pubkey is just 32 bytes.
// Encoding it as raw bytes means neither side needs to agree on which version
// of the Solana SDK the other compiled against. The validator runs against
// Agave's SDK tree; the sim-server runs against whatever SDK versions the DEX
// program crates require. They can be completely different. The wire format
// does not care — it is just bytes on a Unix domain socket.
//
// Frame format
// ────────────
// Every message on the socket is length-prefixed:
//
//   [ u32 LE: payload byte length ][ bincode-serialized payload ]
//
// The length prefix lets the reader allocate exactly the right buffer before
// reading the payload, without scanning for a delimiter. Bincode produces
// compact binary output with no padding or schema overhead, keeping the
// per-request frame small enough that serialization cost is negligible
// relative to the Unix socket round-trip (~5–20 µs on the same machine).

use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing::warn;

// ── Wire types ────────────────────────────────────────────────────────────────

/// Identifies which DEX simulator the sim-server should invoke for a given hop.
///
/// Discriminant values are stable across builds. Bincode serializes this enum
/// using its numeric discriminant, so existing variants must never be renumbered.
/// New DEX kinds must always be appended at the end of the list.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DexKind {
    OrcaWhirlpool = 0,
    RaydiumClmm   = 1,
    RaydiumCp     = 2,
    RaydiumAmmV4  = 3,
    MeteoraDammV2 = 4,
    MeteoraDlmm   = 5,
    PumpAmm       = 6,
    ByrealClmm    = 7,
    PancakeSwap   = 8,
    // MeteoraDammV1 = 9 — suspended: its vault-program account structure was
    // redesigned and no simulator exists for it. Pairs containing a MeteoraDamm
    // v1 pool are filtered out in fangzhen_jieduan before reaching this layer.
}

/// A single on-chain account carried across the process boundary.
///
/// The validator snapshots each required account from the live Bank at the
/// moment a pool update fires and packs it into this struct. The sim-server
/// deserializes the list into an AccountMap keyed by pubkey, then the DEX
/// simulator functions read pool state, vault balances, tick arrays, and
/// configuration accounts from it by key lookup.
///
/// The `owner` field is the program that owns the account on-chain. Several
/// simulators branch on it — Pump and Meteora DLMM distinguish Token-2022
/// accounts from classic SPL token accounts by checking whether the owner is
/// the Token-2022 program ID or the classic SPL Token program ID. The validator
/// reads the real owner from Bank::get_account() and encodes it here so the
/// sim-server can replicate that branch without any RPC call.
///
/// Lamports are not carried because no simulator inspects the lamport balance
/// of any account. Omitting them reduces frame size for every request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WireAccount {
    /// The account's public key encoded as 32 raw bytes.
    pub pubkey: [u8; 32],
    /// The program that owns this account encoded as 32 raw bytes.
    pub owner:  [u8; 32],
    /// The full raw account data bytes as returned by Bank::get_account().
    pub data:   Vec<u8>,
}

/// Sent from the Agave validator → sim-server for every ternary-search probe.
///
/// Each request describes a complete two-hop arbitrage path: the DEX kind,
/// pool address, and input token for each hop, plus every on-chain account
/// both simulators will need. The sim-server is stateless — it reads only from
/// the accounts supplied in this struct and returns the result immediately
/// without caching anything between requests.
///
/// Sending all accounts in-band keeps the sim-server fully stateless. Each
/// request is self-contained and can be handled by any connection thread
/// without shared state, which is why concurrent shard requests never
/// interfere with each other.
#[derive(Serialize, Deserialize, Debug)]
pub struct TwoHopSimRequest {
    /// The SOL amount being tested for this probe, in lamports.
    /// The ternary search in fangzhen_jieduan varies this value across
    /// calls to locate the amount_in that maximises profit.
    pub initial_amount_in: u64,

    /// The slot number at which the triggering pool update was observed.
    /// Passed to simulator functions that need it for TWAP calculations
    /// or time-gated state (e.g. Raydium CLMM observation state).
    pub slot: u64,

    /// Unix timestamp in whole seconds at the moment this request was sent.
    /// Simulators that enforce a pool open_time before allowing swaps
    /// (e.g. Byreal CLMM, Raydium CLMM) use this to gate execution.
    pub unix_timestamp: u64,

    // ── Hop 1: SOL → intermediate token ─────────────────────────────────────

    /// Which DEX simulator to invoke for the first hop.
    pub hop1_dex: DexKind,
    /// Pool address for the first hop encoded as 32 raw bytes.
    pub hop1_pool: [u8; 32],
    /// Token being sold into the first pool encoded as 32 raw bytes.
    /// For a SOL-routed two-hop arb this is always the native SOL mint.
    pub hop1_token_in: [u8; 32],

    // ── Hop 2: intermediate token → SOL ─────────────────────────────────────

    /// Which DEX simulator to invoke for the second hop.
    pub hop2_dex: DexKind,
    /// Pool address for the second hop encoded as 32 raw bytes.
    pub hop2_pool: [u8; 32],
    /// Token being sold into the second pool encoded as 32 raw bytes.
    /// This is whatever intermediate token hop 1 produced — the non-SOL
    /// token that connects the two pools in the arbitrage path.
    pub hop2_token_in: [u8; 32],

    /// All accounts required by both hop simulators, deduplicated by the
    /// validator before sending. When both pools share an account (e.g. both
    /// reference the same token mint) the validator's FxHashSet deduplication
    /// in fangzhen_jieduan ensures only one copy is sent. Duplicates would
    /// waste socket bandwidth and sim-server deserialization time without
    /// providing any additional information to either simulator.
    pub accounts: Vec<WireAccount>,
}

/// Returned from the sim-server → Agave validator for every probe.
///
/// `final_out` is what the validator subtracts `initial_amount_in` from to
/// compute profit for this probe. A `final_out` of zero means either hop
/// failed or produced zero output — the validator treats this as a dead path
/// at this amount_in and either continues the ternary search or moves on.
#[derive(Serialize, Deserialize, Debug)]
pub struct TwoHopSimResponse {
    /// Intermediate token amount produced by hop 1, in the token's native
    /// smallest unit. This value is forwarded as amount_in to the hop 2
    /// simulator — the output of one pool becomes the input of the next.
    pub hop1_out: u64,
    /// Final SOL amount recovered after hop 2, in lamports.
    /// Profit is computed as: final_out as i64 - initial_amount_in as i64.
    pub final_out: u64,
}

// ── Zero-copy serialization ───────────────────────────────────────────────────

/// Serialization-only mirror of `TwoHopSimRequest` that borrows the accounts
/// slice instead of owning it.
///
/// Bincode serializes `&[T]` and `Vec<T>` identically: a u64 element count
/// followed by the elements in order. Frames encoded from `TwoHopSimRequestRef`
/// are therefore byte-for-byte identical to frames encoded from
/// `TwoHopSimRequest`, and the sim-server deserializes them into
/// `TwoHopSimRequest` without any modification.
///
/// The validator's ternary search fires up to 82 sim-server queries per
/// qualifying pair, all using the same account snapshot. Previously, each call
/// to `SimClient::query()` cloned the entire `Vec<WireAccount>` before
/// constructing the request struct — a deep copy of all account data buffers
/// (up to ~17 KB per CLMM pool with tick arrays) on every iteration.
/// By using a borrowed slice here, `query()` avoids all those allocations:
/// the data is read once from the caller's slice during `encode_frame` and
/// written directly to the serialization buffer without ever being duplicated.
///
/// Field order must exactly match `TwoHopSimRequest` because bincode encodes
/// struct fields in declaration order without names or tags.
#[derive(Serialize)]
struct TwoHopSimRequestRef<'a> {
    initial_amount_in: u64,
    slot:              u64,
    unix_timestamp:    u64,
    hop1_dex:          DexKind,
    hop1_pool:         [u8; 32],
    hop1_token_in:     [u8; 32],
    hop2_dex:          DexKind,
    hop2_pool:         [u8; 32],
    hop2_token_in:     [u8; 32],
    accounts:          &'a [WireAccount],
}

// ── Frame encoding and decoding ───────────────────────────────────────────────

/// Serialize `value` with bincode and prepend a 4-byte little-endian length
/// prefix, producing a complete frame ready to write to the socket.
///
/// The length prefix tells the remote reader exactly how many bytes to
/// allocate and read for the payload, removing any need for delimiter scanning.
pub fn encode_frame<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let payload = bincode::serialize(value)
        .map_err(|e| anyhow::anyhow!("bincode serialize failed: {}", e))?;

    let len = payload.len() as u32;
    let mut frame = Vec::with_capacity(4 + payload.len());
    frame.extend_from_slice(&len.to_le_bytes());
    frame.extend_from_slice(&payload);
    Ok(frame)
}

/// Read exactly `buf.len()` bytes from `reader`, blocking until all bytes
/// arrive or the connection closes.
///
/// A short read from a Unix socket does not mean the connection is closed —
/// the kernel may deliver data in arbitrarily sized chunks depending on socket
/// buffer pressure and scheduling. This function loops until the buffer is
/// completely filled, matching the framing guarantee that a complete payload
/// always follows the length prefix.
pub fn read_exact<R: std::io::Read>(reader: &mut R, buf: &mut [u8]) -> std::io::Result<()> {
    let mut filled = 0;
    while filled < buf.len() {
        let n = reader.read(&mut buf[filled..])?;
        if n == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "connection closed before frame was complete",
            ));
        }
        filled += n;
    }
    Ok(())
}

/// Read one length-prefixed frame from `reader` and deserialize it as `T`.
///
/// Reads the 4-byte length prefix first, then allocates a buffer of exactly
/// that many bytes and reads the payload into it. Frames larger than 32 MiB
/// are rejected to guard against a corrupt or malicious length prefix that
/// would otherwise cause an enormous allocation and OOM-kill the process.
pub fn decode_frame<T: for<'de> Deserialize<'de>, R: std::io::Read>(
    reader: &mut R,
) -> Result<T> {
    let mut len_buf = [0u8; 4];
    read_exact(reader, &mut len_buf)?;
    let len = u32::from_le_bytes(len_buf) as usize;

    if len > 32 * 1024 * 1024 {
        anyhow::bail!("incoming frame is too large: {} bytes (limit 32 MiB)", len);
    }

    let mut payload = vec![0u8; len];
    read_exact(reader, &mut payload)?;

    bincode::deserialize(&payload)
        .map_err(|e| anyhow::anyhow!("bincode deserialize failed: {}", e))
}

// ── SimClient ─────────────────────────────────────────────────────────────────

/// A persistent, lazily-connected Unix socket client that lives on a single
/// MevShard thread.
///
/// One SimClient is created per shard at engine startup. The connection to the
/// sim-server is established on the first call to `query()` and then reused
/// for every subsequent call in that shard's lifetime. Reusing the connection
/// amortizes the connect syscall cost across all probes — at up to 80 probes
/// per qualifying pair, paying the connection cost per probe would dominate
/// the latency budget.
///
/// All I/O is synchronous and blocking. Each MevShard runs on a dedicated OS
/// thread, so blocking on a socket read here costs only that shard's thread
/// while the OS keeps it off-CPU. There is no async runtime overhead.
///
/// On any I/O failure the client drops the broken socket and retries the same
/// request exactly once on a fresh connection. The single retry transparently
/// handles sim-server restarts and transient OS-level interrupts (EINTR)
/// without the caller needing to detect or handle reconnection itself. A
/// second consecutive failure is returned as an error so the caller can treat
/// the pair as unprofitable for this slot and move on.
pub struct SimClient {
    /// Path to the Unix domain socket the sim-server is bound to.
    socket_path: PathBuf,
    /// The live connected stream. None before the first query, or after any
    /// I/O failure. The next call to stream() will reconnect transparently.
    stream: Option<UnixStream>,
}

impl SimClient {
    /// Create a client pointed at `socket_path`. No connection is established
    /// here — construction is infallible and costs only a path allocation.
    /// The actual socket connect syscall is deferred to the first `query()`.
    pub fn new(socket_path: impl AsRef<Path>) -> Self {
        Self {
            socket_path: socket_path.as_ref().to_path_buf(),
            stream: None,
        }
    }

    /// Return a mutable reference to the live stream, connecting if needed.
    ///
    /// If the stream is absent (first call, or after a disconnect caused by
    /// an I/O error) this performs the Unix socket connect syscall and stores
    /// the resulting stream for reuse. Subsequent calls return the existing
    /// stream without any syscall.
    fn stream(&mut self) -> Result<&mut UnixStream> {
        if self.stream.is_none() {
            let s = UnixStream::connect(&self.socket_path).with_context(|| {
                format!(
                    "sim_client: failed to connect to sim-server socket at {:?}",
                    self.socket_path
                )
            })?;
            self.stream = Some(s);
        }
        Ok(self.stream.as_mut().unwrap())
    }

    /// Drop the current stream so the next call to stream() reconnects.
    ///
    /// Called immediately after any I/O error so the broken socket descriptor
    /// is closed and never reused. The OS reclaims the file descriptor as soon
    /// as the Option<UnixStream> is dropped here.
    fn disconnect(&mut self) {
        self.stream = None;
    }

    /// Send one two-hop simulation request and block until the response arrives.
    ///
    /// `accounts` is borrowed from the caller — the same snapshot is used for
    /// every ternary-search iteration without any deep copy. The slice is
    /// serialized from the borrow directly into the socket write buffer via
    /// `TwoHopSimRequestRef`, which produces byte-identical frames to those
    /// that a `Vec<WireAccount>`-owning `TwoHopSimRequest` would produce.
    ///
    /// On the first I/O error the client disconnects, reconnects, and retries
    /// the same request exactly once. The retry handles sim-server restarts
    /// and transient kernel interrupts transparently. A second failure is
    /// returned to the caller, which treats the pair as unprofitable for this
    /// slot and continues without crashing the shard thread.
    ///
    /// The `accounts` slice carries every on-chain account both simulators
    /// will need, pre-fetched from the Bank by fangzhen_jieduan and
    /// deduplicated before this call. Sending them in-band keeps the sim-server
    /// fully stateless — it reads only what it receives and touches no shared
    /// state, so concurrent requests from different shards never interfere.
    #[allow(clippy::too_many_arguments)]
    pub fn query(
        &mut self,
        slot:              u64,
        unix_timestamp:    u64,
        initial_amount_in: u64,
        hop1_dex:          DexKind,
        hop1_pool:         [u8; 32],
        hop1_token_in:     [u8; 32],
        hop2_dex:          DexKind,
        hop2_pool:         [u8; 32],
        hop2_token_in:     [u8; 32],
        accounts:          &[WireAccount],
    ) -> Result<TwoHopSimResponse> {
        // TwoHopSimRequestRef borrows the accounts slice rather than owning it.
        // encode_frame reads the slice fields in order and writes them to the
        // serialization buffer without allocating a second copy of the data.
        // The resulting bytes are identical to what TwoHopSimRequest would
        // produce, so the sim-server's decode_frame call is completely unaffected.
        let req = TwoHopSimRequestRef {
            initial_amount_in,
            slot,
            unix_timestamp,
            hop1_dex,
            hop1_pool,
            hop1_token_in,
            hop2_dex,
            hop2_pool,
            hop2_token_in,
            accounts,
        };

        let frame = encode_frame(&req)?;

        match self.try_roundtrip(&frame) {
            Ok(resp) => Ok(resp),
            Err(e) => {
                warn!(
                    "sim_client: roundtrip failed ({}), dropping connection and retrying once",
                    e
                );
                self.disconnect();
                self.try_roundtrip(&frame)
            }
        }
    }

    /// Write the pre-encoded frame to the socket and read back one response.
    ///
    /// write_all guarantees the entire frame is written before returning. The
    /// kernel may split a large write across multiple send() syscalls internally
    /// due to socket buffer limits, but write_all loops until every byte is
    /// delivered or an error occurs. The subsequent decode_frame call then
    /// blocks until the full response frame arrives from the sim-server.
    fn try_roundtrip(&mut self, frame: &[u8]) -> Result<TwoHopSimResponse> {
        let stream = self.stream()?;
        stream
            .write_all(frame)
            .context("sim_client: failed to write request frame to socket")?;
        decode_frame::<TwoHopSimResponse, _>(stream)
            .context("sim_client: failed to read response frame from socket")
    }
}
