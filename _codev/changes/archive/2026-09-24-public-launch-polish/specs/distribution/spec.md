## MODIFIED Requirements

### Requirement: The documentation lists the three installation paths

Note: the historical title keeps "three paths" to preserve name
compatibility with the main spec; the content below describes
**four** paths after Windows was added. A clean rename will come in
a dedicated cycle.

The Installation section of `docs/codev.md` MUST list, in this order:

1. **Recommended Unix path** — `curl -sSL … | sh` for macOS/Linux.
2. **Recommended Windows path** — `iwr -useb … | iex` for Windows
   in PowerShell.
3. **Manual path** — download from GitHub Releases + check with
   `sha256sum -c` (Unix) or `Get-FileHash` (Windows).
4. **Contributor path** — `cargo install --path crates/codev-cli`
   from a clone of the repository.

The repository README MUST mention at least the first **and** the
second path (with the one-liners `curl … | sh` and `iwr … | iex`).

#### Scenario: the Installation section of docs/codev.md lists the four paths in order

- **GIVEN** a reader who opens `docs/codev.md` at the Installation
  section
- **WHEN** they go through the subsections in order
- **THEN** they successively encounter the Unix path (`curl | sh`),
  the Windows path (`iwr | iex`), the manual path (download from
  GitHub Releases with SHA-256 check), and the contributor path
  (`cargo install --path`)

#### Scenario: the README points to at least the two "no Rust" paths in its Getting Started

- **GIVEN** a reader who opens `README.md` at the repository root
- **WHEN** they go through the "Getting Started" section
- **THEN** they see the `curl -sSL … | sh` example for macOS/Linux
- **AND** they see the `iwr -useb … | iex` example for Windows in
  PowerShell
