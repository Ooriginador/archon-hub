# 🖥️ User Interface Guide

> Panels, controls, and interaction patterns

---

## Window Layout

ARKHEION Forge uses a four-panel layout built with egui:

```text
┌─────────────────────────────────────────────────────────────┐
│  File  Edit                                          Status │  ← Top Menu Bar
├───────────┬───────────────────────────────┬──────────────────┤
│           │                               │                  │
│   Gene    │      🌌 Gene Space            │   🔬 Gene Editor │
│   Browser │      (Central Panel)          │   (Right Panel)  │
│   (Left   │                               │                  │
│    Panel)  │      Entropy vs Sparsity     │   Statistics      │
│           │      scatter plot             │   Surgery Tools  │
│           │                               │                  │
├───────────┴───────────────────────────────┴──────────────────┤
│  📝 Edit History (Bottom Panel)                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Top Menu Bar

### File Menu

| Action | Description |
| ------ | ----------- |
| 📂 Open .nucleus / .arktern | Native file dialog, filtered to supported formats |
| 💾 Save .nucleus | Saves current pool to `*_edited.nucleus` |

### Edit Menu

| Action | Description |
| ------ | ----------- |
| ↩ Undo last edit | Reserved for Phase 2 |

### Status Bar

The right side of the menu bar shows contextual status messages:

```text
Ready. Open a .nucleus or .arktern file to begin.
Loading /path/to/model.arktern...
Loaded 67 genes | 1235814400 params | 21.3% sparse | 1.42 bits entropy
Pruned: 16777216 nnz → 8388608 nnz
Save error: Permission denied
```

---

## 🧬 Gene Browser (Left Panel)

**Width**: 280px minimum, resizable

### Search Bar

```text
🔍 [________________]
```

Type to filter genes by layer name (case-insensitive substring match).

### Domain Filter Buttons

```text
[ALL] [Attention] [Mlp] [Norm] [Embed] [Output]
```

Click to filter by domain. `ALL` shows everything. Active filter is highlighted.

### Fragile Filter

```text
☐ ⚠️ Fragile only
```

When checked, shows only Norm and Embed genes (the dangerous ones).

### Gene List

Scrollable list of all genes matching current filters:

```text
Showing 67/67
  blk.0.attn_k.weight [Attention]       ← Red colored
  blk.0.attn_q.weight [Attention]       ← Red colored
  blk.0.attn_v.weight [Attention]       ← Red colored
⚠️ blk.0.attn_norm.weight [Norm]        ← Blue colored, fragile marker
  blk.0.ffn_down.weight [Mlp]           ← Teal colored
  blk.0.ffn_gate.weight [Mlp]           ← Teal colored
  blk.0.ffn_up.weight [Mlp]             ← Teal colored
  ...
```

- **Click** a gene to select it (opens in Gene Editor)
- **Colors** match the domain (see [GENE_MODEL.md](GENE_MODEL.md))
- **⚠️** prefix indicates fragile layers
- **Font size**: 11pt for compact display
- **Sorted** alphabetically by layer name

---

## 🔬 Gene Editor (Right Panel)

**Width**: 300px minimum, resizable

### Gene Information Box

When a gene is selected:

```text
┌────────────────────────────┐
│ blk.0.attn_q.weight       │  ← Bold, 13pt
│ ID: a1b2c3d4e5f6          │  ← First 12 chars
│ Shape: [4096, 4096]        │
│ Elements:        16.8M     │  ← Formatted
│ Packed:          3.4 MB    │  ← Formatted
│ Domain: Attention          │
└────────────────────────────┘
```

### Statistics Section

```text
📊 Statistics
┌────────────────────────────┐
│ φ Quality:  0.8340         │
│ Entropy:    1.4237 bits    │
│ Sparsity:   21.3%          │
│ Distribution: -1:3.2M | 0:3.6M | +1:3.2M │
│ ⚠️ FRAGILE LAYER — edit   │  ← Only for Norm/Embed
│     with care!             │
└────────────────────────────┘
```

### Surgery Tools

```text
🛠️ Surgery Tools

Prune ratio:  [====|=====] 0.10
[✂️ Prune]

Mutate prob:  [=|========] 0.01
[🧬 Mutate]

[🗑️ Amputate (zero all)]      ← Red text
```

- **Prune slider**: 0.0 – 0.5 (50% max)
- **Mutate slider**: 0.001 – 0.1 (10% max)
- **Amputate button**: Red colored as warning

Each button triggers the corresponding operation on the selected gene and appends an `EditResult` to history.

---

## 🌌 Gene Space (Central Panel)

### With Model Loaded

A 2D scatter plot showing all genes positioned by their statistical properties:

```text
Sparsity ↑
    │
    │       · ·    · 
    │    ·    · ·     ·
    │  ·    ·      ·
    │     ·   · ·
    │       ·   ·
    │
    └──────────────────── Entropy (bits) →
```

- **X axis**: Entropy (0 – 1.6 bits)
- **Y axis**: Sparsity (0% – 100%)
- **Point color**: Domain color (red=Attention, teal=MLP, etc.)
- **Point size**: 8px for fragile, 4px for normal
- **Background**: Dark (#0A0A1A)

### Without Model

Welcome screen with open button:

```text
        🧬 ARKHEION Forge
        AI Model Editor

    [📂 Open File]
```

---

## 📝 Edit History (Bottom Panel)

**Height**: 80px minimum, resizable

Horizontal scrollable log of recent edits (newest first, max 20 visible):

```text
📝 Edit History
┌──────┬──────────┬──────────┐──────┬──────────┬──────────┐
│  #2  │  #1      │  #0      │
│ prune│ mutate   │ prune    │
│ 8.4M │ 8.4M →   │ 16.8M → │
│ → 4.2M│ 8.3M    │ 8.4M    │
└──────┴──────────┴──────────┘
```

Each card shows:
- Edit number (sequential)
- Operation name
- Before → after nnz counts

---

## Keyboard Shortcuts (Planned)

| Shortcut | Action |
| -------- | ------ |
| Ctrl+O | Open file |
| Ctrl+S | Save .nucleus |
| Ctrl+Z | Undo (Phase 2) |
| F | Toggle fragile filter |
| 1-5 | Switch domain filter |
| Esc | Deselect gene |

---

## Color Reference

| Element | Color | Hex |
| ------- | ----- | --- |
| Attention genes | Red | #FF6B6B |
| MLP genes | Teal | #4ECDC4 |
| Norm genes | Blue | #45B7D1 |
| Embed genes | Green | #96CEB4 |
| Output genes | Yellow | #FFEAA7 |
| Bias genes | Plum | #DDA0DD |
| Conv genes | Orange | #FF8C42 |
| Unknown genes | Gray | #888888 |
| Background | Dark navy | #0A0A1A |
| Axes text | Gray | #646464 |
| Fragile warning | Yellow | egui::Color32::YELLOW |
| Amputate button | Red | egui::Color32::RED |

---

*Next: [INTEGRATION.md](INTEGRATION.md) — Python pipeline integration*
