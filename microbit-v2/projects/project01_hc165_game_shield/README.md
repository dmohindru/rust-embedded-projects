## Introduction

This project demo the usage of [74HC165](../../../embedded-core/src/shift_register/hc165/mod.rs) parallel to serial shift register. Demo was done for [this board](https://shop.elecfreaks.com/products/micro-bit-retro-programming-arcade?pr_prod_strat=e5_desc&pr_rec_id=2e61111b4&pr_rec_pid=7000691277903&pr_ref_pid=6851041755215&pr_seq=uniform)

## Pin Mapping

### Microbit-v2 pins

Arcade Shield INSR_LATCH
micro:bit edge connector P9
nRF52833 P0.09
| Shield Pin | Micro:bit edge pin | nRF52833 pin |
|------------|--------------------|--------------|
| INSR_LATCH | P9 | P0.09 |
| SR_CLK | P20 | P1.00 |
| INSR0_DATA | P14 | P0.01 |

---

### Serial input data bitmaps

**Byte 0 - Button data**
|bit n| button |
|-----|--------|
| 7 | GND |
| 6 | menu |
| 5 | btn_b |
| 4 | btn_a |
| 3 | btn_right |
| 2 | btn_down |
| 1 | btn_up |
| 0 | btn_left |

---

**Byte 1 - Config data**

- Value: 0x30
- HW_CFG3 = 0
- HW_CFG2 = 0
- HW_CFG1 = 1 -> ST7735 (CFG1=0x603 or 0x12c2d)
- HW_CFG0 = 1 -> Not rotated
