# pngme
implementation of the learning project 'PNGme'

## The PNG file format

A PNG file is a header (8 bytes) (always the same), followed by a list of chunks.

### Chunks
PNG files are (essentially) a list of 'chunks', each containing its own data.
Each chunk has a type, represented by a 4 character string.
There are standard types, such as image data, but we can also implement our own types, thus letting us insert whatever data we would like -- we can even tell other PNG decoders to ignore the chunks we define.

### Chunk Layout

- Length (4 bytes)
- Chunk Type (4 bytes -- restricted to uppercase (65-90) and lowercase(97-122) ASCII)
- Chunk Data (0+)
- CRC (4 bytes)

#### Valid Chunks 
Four bits of the type code (bit 5 of each byte) are used to convey a chunk's property.
If the bit is 0, the letter of the type code is uppercase.
If the bit is 1, the letter of the type code is lowercase.

##### Ancillary bit (bit#5 of byte#1)
0 = critical
1 = ancillary

##### Private bit (bit#5 of byte#2)
0 = public
1 = private 

##### Reserved bit (bit#5 of byte#3)
Must be 0

##### Safe-to-copy bit (byte#5 of byte#4)
0 = unsafe to copy 
1 = safe to copy 
