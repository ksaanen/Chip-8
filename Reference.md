# Technical Reference
This is a conversion of [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM) to markdown format, all credits about the content of this document go to him (Thomas P. Greene).

## Table of Contents <span id="toc">

- [Using This Document](#using_this_document)
- [About Chip-8](#about_chip8)
- [Chip-8 Specifications](#chip8_specifications)
- [Memory Diagram - Memory Map](#memory_diagram)
- [Registers](#registers)
- [Keyboard Diagram - Keyboard Layout](#keyboard_diagram)
- [Display Diagram - Display Coordinates Listing - The - Chip-8 Hexadecimal Font](#display_diagram)
- [Timers & Sound](#timers_and_sound)
- [Chip-8 Instructions](#chip8_instructions)
- [Standard Chip-8 Instructions](#standard_chip8_instructions)
  - [00E0](#00E0) - CLS
  - [00EE](#00EE) - RET
  - [0nnn](#0nnn) - SYS addr
  - [1nnn](#1nnn) - JP addr
  - [2nnn](#2nnn) - CALL addr
  - [3xkk](#3xkk) - SE Vx, byte
  - [4xkk](#4xkk) - SNE Vx, byte
  - [5xy0](#5xy0) - SE Vx, Vy
  - [6xkk](#6xkk) - LD Vx, byte
  - [7xkk](#7xkk) - ADD Vx, byte
  - [8xy0](#8xy0) - LD Vx, Vy
  - [8xy1](#8xy1) - OR Vx, Vy
  - [8xy2](#8xy2) - AND Vx, Vy
  - [8xy3](#8xy3) - XOR Vx, Vy
  - [8xy4](#8xy4) - ADD Vx, Vy
  - [8xy5](#8xy5) - SUB Vx, Vy
  - [8xy6](#8xy6) - SHR Vx {, Vy}
  - [8xy7](#8xy7) - SUBN Vx, Vy
  - [8xyE](#8xyE) - SHL Vx {, Vy}
  - [9xy0](#9xy0) - SNE Vx, Vy
  - [Annn](#Annn) - LD I, addr
  - [Bnnn](#Bnnn) - JP V0, addr
  - [Cxkk](#Cxkk) - RND Vx, byte
  - [Dxyn](#Dxyn) - DRW Vx, Vy, nibble
  - [Ex9E](#Ex9E) - SKP Vx
  - [ExA1](#ExA1) - SKNP Vx
  - [Fx07](#Fx07) - LD Vx, DT
  - [Fx0A](#Fx0A) - LD Vx, K
  - [Fx15](#Fx15) - LD DT, Vx
  - [Fx18](#Fx18) - LD ST, Vx
  - [Fx1E](#Fx1E) - ADD I, Vx
  - [Fx29](#Fx29) - LD F, Vx
  - [Fx33](#Fx33) - LD B, Vx
  - [Fx55](#Fx55) - LD \[I\], Vx
  - [Fx65](#Fx65) - LD Vx, \[I\]
- [Super Chip-48 Instructions](#super_chip48_instructions)
  - [00Cn](#super_chip48_instructions) - SCD nibble
  - [00FB](#super_chip48_instructions) - SCR
  - [00FC](#super_chip48_instructions) - SCL
  - [00FD](#super_chip48_instructions) - EXIT
  - [00FE](#super_chip48_instructions) - LOW
  - [00FF](#super_chip48_instructions) - HIGH
  - [Dxy0](#super_chip48_instructions) - DRW Vx, Vy, 0
  - [Fx30](#super_chip48_instructions) - LD HF, Vx
  - [Fx75](#super_chip48_instructions) - LD R, Vx
  - [Fx85](#super_chip48_instructions) - LD Vx, R
- [Interpreters](#interpreters)
- [Credits](#credits)

## Using This Document <span id="using_this_document"></span>[\[TOC\]](#toc)

While creating this document, I took every effort to try to make it easy to read, as well as easy to find what you're looking for. In most cases, where a hexadecimal value is given, it is followed by the equivalent decimal value in parenthesis. For example, "0x200 (512)." In most cases, when a word or letter is italicized, it is referring to a variable value, for example, if I write "Vx," the x reffers to a 4-bit value. The most important thing to remember as you read this document is that every [TOC] link will take you back to the Table Of Contents. Also, links that you have not yet visited will appear in blue, while links you have used will be gray.

## About Chip-8 <span id="about_chip8"></span>[\[TOC\]](#toc)

Whenever I mention to someone that I'm writing a Chip-8 interpreter, the response is always the same: "What's a Chip-8?" Chip-8 is a simple, interpreted, programming language which was first used on some do-it-yourself computer systems in the late 1970s and early 1980s. The COSMAC VIP, DREAM 6800, and ETI 660 computers are a few examples. These computers typically were designed to use a television as a display, had between 1 and 4K of RAM, and used a 16-key hexadecimal keypad for input. The interpreter took up only 512 bytes of memory, and programs, which were entered into the computer in hexadecimal, were even smaller. In the early 1990s, the Chip-8 language was revived by a man named Andreas Gustafsson. He created a Chip-8 interpreter for the HP48 graphing calculator, called Chip-48. The HP48 was lacking a way to easily make fast games at the time, and Chip-8 was the answer. Chip-48 later begat Super Chip-48, a modification of Chip-48 which allowed higher resolution graphics, as well as other graphical enhancements. Chip-48 inspired a whole new crop of Chip-8 interpreters for various platforms, including MS-DOS, Windows 3.1, Amiga, HP48, MSX, Adam, and ColecoVision. I became involved with Chip-8 after stumbling upon Paul Robson's interpreter on the World Wide Web. Shortly after that, I began writing my own Chip-8 interpreter. This document is a compilation of all the different sources of information I used while programming my interpreter.

## Chip-8 Specifications <span id="chip8_specifications"></span>[\[TOC\]](#toc)

This section describes the Chip-8 memory, registers, display, keyboard, and timers.

## Memory <span id="memory_diagram"></span>[\[TOC\]](#toc)

The Chip-8 language is capable of accessing up to 4KB (4,096 bytes) of RAM, from location 0x000 (0) to 0xFFF (4095). The first 512 bytes, from 0x000 to 0x1FF, are where the original interpreter was located, and should not be used by programs. Most Chip-8 programs start at location 0x200 (512), but some begin at 0x600 (1536). Programs beginning at 0x600 are intended for the ETI 660 computer.

```
Memory Map:
+---------------+= 0xFFF (4095) End of Chip-8 RAM
|               |
|               |
|               |
|               |
|               |
| 0x200 to 0xFFF|
|    Chip-8     |
| Program / Data|
|    Space      |
|               |
|               |
|               |
+- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
|               |
|               |
|               |
+---------------+= 0x200 (512) Start of most Chip-8 programs
| 0x000 to 0x1FF|
| Reserved for  |
| interpreter   |
+---------------+= 0x000 (0) Start of Chip-8 RAM
```

## Registers <span id="registers"></span>[\[TOC\]](#toc)

Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit (0 through F). There is also a 16-bit register called I. This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used. The VF register should not be used by any program, as it is used as a flag by some instructions. See section 3.0, Instructions for details. Chip-8 also has two special purpose 8-bit registers, for the delay and sound timers. When these registers are non-zero, they are automatically decremented at a rate of 60Hz. See the section 2.5, Timers & Sound, for more information on these. There are also some "pseudo-registers" which are not accessable from Chip-8 programs. The program counter (PC) should be 16-bit, and is used to store the currently executing address. The stack pointer (SP) can be 8-bit, it is used to point to the topmost level of the stack. The stack is an array of 16 16-bit values, used to store the address that the interpreter shoud return to when finished with a subroutine. Chip-8 allows for up to 16 levels of nested subroutines.

## Keyboard <span id="keyboard_diagram"></span>[\[TOC\]](#toc)

The computers which originally used the Chip-8 Language had a 16-key hexadecimal keypad with the following layout:

|     |     |     |     |
| --- | --- | --- | --- |
| 1   | 2   | 3   | C   |
| 4   | 5   | 6   | D   |
| 7   | 8   | 9   | E   |
| A   | 0   | B   | F   |

This layout must be mapped into various other configurations to fit the
keyboards of today's platforms.

## Display <span id="display_diagram"></span>[\[TOC\]](#toc)

The original implementation of the Chip-8 language used a 64x32-pixel monochrome display with this format:

|        |         |
| ------ | ------- |
| (0,0)  | (63,0)  |
| (0,31) | (63,31) |

Some other interpreters, most notably the one on the ETI 660, also had
64x48 and 64x64 modes. To my knowledge, no current interpreter supports
these modes. More recently, Super Chip-48, an interpreter for the HP48
calculator, added a 128x64-pixel mode. This mode is now supported by
most of the interpreters on other platforms.

Chip-8 draws graphics on screen through the use of sprites. A sprite is
a group of bytes which are a binary representation of the desired
picture. Chip-8 sprites may be up to 15 bytes, for a possible sprite
size of 8x15.

Programs may also refer to a group of sprites representing the
hexadecimal digits 0 through F. These sprites are 5 bytes long, or 8x5
pixels. The data should be stored in the interpreter area of Chip-8
memory (0x000 to 0x1FF). Below is a listing of each character's bytes,
in binary and hexadecimal:

| 0      | Binary   | Hex  |
| ------ | -------- | ---- |
| `####` | 11110000 | 0xF0 |
| `#__#` | 10010000 | 0x90 |
| `#__#` | 10010000 | 0x90 |
| `#__#` | 10010000 | 0x90 |
| `####` | 11110000 | 0xF0 |

| 1      | Binary   | Hex  |
| ------ | -------- | ---- |
| `__#_` | 00100000 | 0x20 |
| `_##_` | 01100000 | 0x60 |
| `__#_` | 00100000 | 0x20 |
| `__#_` | 00100000 | 0x20 |
| `_###` | 01110000 | 0x70 |

| 2      | Binary   | Hex  |
| ------ | -------- | ---- |
| `####` | 11110000 | 0xF0 |
| `___#` | 00010000 | 0x10 |
| `####` | 11110000 | 0xF0 |
| `#___` | 10000000 | 0x80 |
| `####` | 11110000 | 0xF0 |

| 3      | Binary   | Hex  |
| ------ | -------- | ---- |
| `####` | 11110000 | 0xF0 |
| `___#` | 00010000 | 0x10 |
| `####` | 11110000 | 0xF0 |
| `___#` | 00010000 | 0x10 |
| `####` | 11110000 | 0xF0 |

| 4      | Binary   | Hex  |
| ------ | -------- | ---- |
| `#__#` | 10010000 | 0x90 |
| `#__#` | 10010000 | 0x90 |
| `####` | 11110000 | 0xF0 |
| `___#` | 00010000 | 0x10 |
| `___#` | 00010000 | 0x10 |

| 5      | Binary   | Hex  |
| ------ | -------- | ---- |
| `####` | 11110000 | 0xF0 |
| `#___` | 10000000 | 0x80 |
| `####` | 11110000 | 0xF0 |
| `___#` | 00010000 | 0x10 |
| `####` | 11110000 | 0xF0 |

| 6      | Binary   | Hex  |
| ------ | -------- | ---- |
| `####` | 11110000 | 0xF0 |
| `#___` | 10000000 | 0x80 |
| `####` | 11110000 | 0xF0 |
| `#__#` | 10010000 | 0x90 |
| `####` | 11110000 | 0xF0 |

| 7      | Binary   | Hex  |
| ------ | -------- | ---- |
| `####` | 11110000 | 0xF0 |
| `___#` | 00010000 | 0x10 |
| `__#_` | 00100000 | 0x20 |
| `_#__` | 01000000 | 0x40 |
| `_#__` | 01000000 | 0x40 |

| 8      | Binary   | Hex  |
| ------ | -------- | ---- |
| `####` | 11110000 | 0xF0 |
| `#__#` | 10010000 | 0x90 |
| `####` | 11110000 | 0xF0 |
| `#__#` | 10010000 | 0x90 |
| `####` | 11110000 | 0xF0 |

| 9      | Binary   | Hex  |
| ------ | -------- | ---- |
| `####` | 11110000 | 0xF0 |
| `#__#` | 10010000 | 0x90 |
| `####` | 11110000 | 0xF0 |
| `___#` | 00010000 | 0x10 |
| `####` | 11110000 | 0xF0 |

| A      | Binary   | Hex  |
| ------ | -------- | ---- |
| `####` | 11110000 | 0xF0 |
| `#__#` | 10010000 | 0x90 |
| `####` | 11110000 | 0xF0 |
| `#__#` | 10010000 | 0x90 |
| `#__#` | 10010000 | 0x90 |

| B      | Binary   | Hex  |
| ------ | -------- | ---- |
| `###_` | 11100000 | 0xE0 |
| `#__#` | 10010000 | 0x90 |
| `###_` | 11100000 | 0xE0 |
| `#__#` | 10010000 | 0x90 |
| `###_` | 11100000 | 0xE0 |

| C      | Binary   | Hex  |
| ------ | -------- | ---- |
| `####` | 11110000 | 0xF0 |
| `#___` | 10000000 | 0x80 |
| `#___` | 10000000 | 0x80 |
| `#___` | 10000000 | 0x80 |
| `####` | 11110000 | 0xF0 |

| D      | Binary   | Hex  |
| ------ | -------- | ---- |
| `###_` | 11100000 | 0xE0 |
| `#__#` | 10010000 | 0x90 |
| `#__#` | 10010000 | 0x90 |
| `#__#` | 10010000 | 0x90 |
| `###_` | 11100000 | 0xE0 |

| E      | Binary   | Hex  |
| ------ | -------- | ---- |
| `####` | 11110000 | 0xF0 |
| `#___` | 10000000 | 0x80 |
| `####` | 11110000 | 0xF0 |
| `#___` | 10000000 | 0x80 |
| `####` | 11110000 | 0xF0 |

| F      | Binary   | Hex  |
| ------ | -------- | ---- |
| `####` | 11110000 | 0xF0 |
| `#___` | 10000000 | 0x80 |
| `####` | 11110000 | 0xF0 |
| `#___` | 10000000 | 0x80 |
| `#___` | 10000000 | 0x80 |

## Timers & Sound <span id="timers_and_sound"></span>[\[TOC\]](#toc)

Chip-8 provides 2 timers, a delay timer and a sound timer. The delay timer is active whenever the delay timer register (DT) is non-zero. This timer does nothing more than subtract 1 from the value of DT at a rate of 60Hz. When DT reaches 0, it deactivates. The sound timer is active whenever the sound timer register (ST) is non-zero. This timer also decrements at a rate of 60Hz, however, as long as ST's value is greater than zero, the Chip-8 buzzer will sound. When ST reaches zero, the sound timer deactivates. The sound produced by the Chip-8 interpreter has only one tone. The frequency of this tone is decided by the author of the interpreter.

## Chip-8 Instructions <span id="chip8_instructions"></span>[\[TOC\]](#toc)

The original implementation of the Chip-8 language includes 36 different instructions, including math, graphics, and flow control functions. Super Chip-48 added an additional 10 instructions, for a total of 46. All instructions are 2 bytes long and are stored most-significant-byte first. In memory, the first byte of each instruction should be located at an even addresses. If a program includes sprite data, it should be padded so any instructions following it will be properly situated in RAM. This document does not yet contain descriptions of the Super Chip-48 instructions. They are, however, listed below. In these listings, the following variables are used: nnn or addr - A 12-bit value, the lowest 12 bits of the instruction n or nibble - A 4-bit value, the lowest 4 bits of the instruction x - A 4-bit value, the lower 4 bits of the high byte of the instruction y - A 4-bit value, the upper 4 bits of the low byte of the instruction kk or byte - An 8-bit value, the lowest 8 bits of the instruction

## Standard Chip-8 Instructions <span id="standard_chip8_instructions"></span>[\[TOC\]](#toc)

### 0nnn - SYS addr <span id="0nnn"></span>

Jump to a machine code routine at nnn.

This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.

### 00E0 - CLS <span id="00E0"></span>

Clear the display.

### 00EE - RET <span id="00EE"></span>

Return from a subroutine.

The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.

### 1nnn - JP addr <span id="1nnn"></span>

Jump to location nnn.

The interpreter sets the program counter to nnn.

### 2nnn - CALL addr <span id="2nnn"></span>

Call subroutine at nnn.

The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.

### 3xkk - SE Vx, byte <span id="3xkk"></span>

Skip next instruction if Vx = kk.

The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.

### 4xkk - SNE Vx, byte <span id="4xkk"></span>

Skip next instruction if Vx != kk.

The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.

### 5xy0 - SE Vx, Vy <span id="5xy0"></span>

Skip next instruction if Vx = Vy.

The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.

### 6xkk - LD Vx, byte <span id="6xkk"></span>

Set Vx = kk.

The interpreter puts the value kk into register Vx.

### 7xkk - ADD Vx, byte <span id="7xkk"></span>

Set Vx = Vx + kk.

Adds the value kk to the value of register Vx, then stores the result in Vx.

### 8xy0 - LD Vx, Vy <span id="8xy0"></span>

Set Vx = Vy.

Stores the value of register Vy in register Vx.

### 8xy1 - OR Vx, Vy <span id="8xy1"></span>

Set Vx = Vx OR Vy.

Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.

### 8xy2 - AND Vx, Vy <span id="8xy2"></span>

Set Vx = Vx AND Vy.
Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.

### 8xy3 - XOR Vx, Vy <span id="8xy3"></span>

Set Vx = Vx XOR Vy.
Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.

### 8xy4 - ADD Vx, Vy <span id="8xy4"></span>

Set Vx = Vx + Vy, set VF = carry. The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.

### 8xy5 - SUB Vx, Vy <span id="8xy5"></span>

Set Vx = Vx - Vy, set VF = NOT borrow. If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.

### 8xy6 - SHR Vx {, Vy} <span id="8xy6"></span>

Set Vx = Vx SHR 1. If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.

### 8xy7 - SUBN Vx, Vy <span id="8xy7"></span>

Set Vx = Vy - Vx, set VF = NOT borrow. If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.

### 8xyE - SHL Vx {, Vy} <span id="8xyE"></span>

Set Vx = Vx SHL 1. If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.

### 9xy0 - SNE Vx, Vy <span id="9xy0"></span>

Skip next instruction if Vx != Vy. The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.

### Annn - LD I, addr <span id="Annn"></span>

Set I = nnn. The value of register I is set to nnn.

### Bnnn - JP V0, addr <span id="Bnnn"></span>

Jump to location nnn + V0. The program counter is set to nnn plus the value of V0.

### Cxkk - RND Vx, byte <span id="Cxkk"></span>

Set Vx = random byte AND kk. The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.

### Dxyn - DRW Vx, Vy, nibble <span id="Dxyn"></span>

Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.

### Ex9E - SKP Vx <span id="Ex9E"></span>

Skip next instruction if key with the value of Vx is pressed. Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.

### ExA1 - SKNP Vx <span id="ExA1"></span>

Skip next instruction if key with the value of Vx is not pressed. Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.

### Fx07 - LD Vx, DT <span id="Fx07"></span>

Set Vx = delay timer value. The value of DT is placed into Vx.

### Fx0A - LD Vx, K <span id="Fx0A"></span>

Wait for a key press, store the value of the key in Vx. All execution stops until a key is pressed, then the value of that key is stored in Vx.

### Fx15 - LD DT, Vx <span id="Fx15"></span>

Set delay timer = Vx. DT is set equal to the value of Vx.

### Fx18 - LD ST, Vx <span id="Fx18"></span>

Set sound timer = Vx. ST is set equal to the value of Vx.

### Fx1E - ADD I, Vx <span id="Fx1E"></span>

Set I = I + Vx. The values of I and Vx are added, and the results are stored in I.

### Fx29 - LD F, Vx <span id="Fx29"></span>

Set I = location of sprite for digit Vx. The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.

## Fx33 - LD B, Vx <span id="Fx33"></span>

Store BCD representation of Vx in memory locations I, I+1, and I+2. The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.

### Fx55 - LD \[I\], Vx <span id="Fx55"></span>

Store registers V0 through Vx in memory starting at location I. The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.

### Fx65 - LD Vx, \[I\] <span id="Fx65"></span>

Read registers V0 through Vx from memory starting at location I. The interpreter reads values from memory starting at location I into registers V0 through Vx.

## Super Chip-48 Instructions <span id="super_chip48_instructions"></span>[\[TOC\]](#toc)

```
00Cn - SCD nibble
00FB - SCR
00FC - SCL
00FD - EXIT
00FE - LOW
00FF - HIGH
Dxy0 - DRW Vx, Vy, 0
Fx30 - LD HF, Vx
Fx75 - LD R, Vx
Fx85 - LD Vx, R
```

## Interpreters <span id="interpreters"></span>[\[TOC\]](#toc)

Below is a list of every Chip-8 interpreter I could find on the World Wide Web:

| Title           | Version | Author                                     | Platform(s)                  |
| --------------- | ------- | ------------------------------------------ | ---------------------------- |
| Chip-48         | 2.20    | Anrdreas Gustafsson                        | HP48                         |
| Chip8           | 1.1     | Paul Robson                                | DOS                          |
| Chip-8 Emulator | 2.0.0   | David Winter                               | DOS                          |
| CowChip         | 0.1     | Thomas P. Greene                           | Windows 3.1                  |
| DREAM MON       | 1.1     | Paul Hayter                                | Amiga                        |
| Super Chip-48   | 1.1     | Based on Chip-48, modified by Erik Bryntse | HP48                         |
| Vision-8        | 1.0     | Marcel de Kogel                            | DOS, Adam, MSX, ColecoVision |

## Credits <span id="credits"></span>[\[TOC\]](#toc)

This document was compiled by Thomas P. Greene. Sources include:

- My own hacking.
- E-mail between David Winter and myself.
- David Winter's Chip-8 Emulator documentation.
- Christian Egeberg's Chipper documentation.
- Marcel de Kogel's Vision-8 source code.
- Paul Hayter's DREAM MON documentation.
- Paul Robson's web page.
- Andreas Gustafsson's Chip-48 documentation.
