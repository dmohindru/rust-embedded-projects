# Toy Filesystem Design

## Goal

Implement a simple educational filesystem that can run unchanged on:

- RAM disk
- SPI NOR Flash (W25Q128)
- I2C EEPROM
- SD Card

The goal is to understand how real filesystems separate storage management from hardware access.

---

# Design Principles

1. Filesystem knows only blocks.
2. Filesystem never talks to SPI, I2C, or SDIO directly.
3. Storage-specific behavior is hidden behind lower layers.
4. Simplicity is preferred over performance.

---

# On-Disk Layout

Assume:

- Block size = 4096 bytes
- Total blocks = device dependent

Layout:

| Block Range | Purpose |
|------------|---------|
| 0 | Superblock |
| 1-8 | Directory Table |
| 9+ | File Data |

---

# Superblock

Stored in Block 0.

Example:

```text
Magic      = TOYFS
Version    = 1
BlockSize  = 4096
TotalBlocks= 4096
```

Purpose:

- Verify filesystem exists
- Validate compatibility
- Discover storage geometry

---

# Directory Entry

Fixed-size entry.

```c
struct DirectoryEntry {
    char     name[32];
    uint32_t start_block;
    uint32_t file_size;
    uint8_t  in_use;
};
```

Advantages:

- Easy to implement
- Easy to inspect in debugger
- Deterministic memory usage

---

# File Allocation

A file occupies consecutive blocks.

Example:

hello.txt

```text
start_block = 10
file_size   = 6000 bytes
```

Occupies:

```text
Block 10
Block 11
```

because 6000 > 4096.

---

# Example Walkthrough

## Format

1. Write superblock.
2. Clear directory table.
3. Mark remaining blocks free.

---

## Create hello.txt

Content:

```text
Hello World
```

Filesystem:

1. Find free directory entry.
2. Find free data block.
3. Write data.
4. Update directory entry.

Directory:

```text
hello.txt -> block 10
```

---

## Read hello.txt

Filesystem:

1. Search directory.
2. Locate start block.
3. Read blocks.
4. Return requested bytes.

---

## Delete hello.txt

Filesystem:

1. Mark directory entry unused.
2. Mark data blocks free.

Data may remain physically present until reused.

---

# Supported Operations

```rust
format()
mount()

create_file()
read_file()
write_file()
delete_file()

list_files()
```

---

# Future Extensions

After basic version works:

- Subdirectories
- Variable length files
- Free block bitmap
- Checksums
- Journaling
- Wear-level awareness
- Power-loss recovery

---

# Educational Outcomes

This project teaches:

- Filesystem metadata
- Allocation strategies
- Mounting
- Formatting
- Fragmentation
- Storage abstraction
- Block-based design
